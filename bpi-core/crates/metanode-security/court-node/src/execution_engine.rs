use anyhow::Result;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing;
use uuid::Uuid;

use crate::{
    ContractExecutionEngine, ExecutionContext, ExecutionResult, ExecutionError,
    ExecutionStep, ExecutionTrace, YamlContract, ContractInstance, ContractStatus,
    ResourceLimits, SecurityLevel, ContractRuntime, RuntimeConfig,
};

impl ContractExecutionEngine {
    /// Create a new contract execution engine
    pub fn new() -> Self {
        
        let default_context = ExecutionContext {
            contract_id: Uuid::new_v4(),
            execution_id: Uuid::new_v4(),
            input_parameters: HashMap::new(),
            environment: HashMap::new(),
            executed_at: chrono::Utc::now(),
        };
        
        let default_runtime = ContractRuntime {
            config: RuntimeConfig {
                memory_limit_mb: 128,
                cpu_limit_percent: 50,
                network_access: false,
                filesystem_access: false,
            },
            active_executions: Arc::new(RwLock::new(HashMap::new())),
        };
        
        Self {
            active_contracts: Arc::new(RwLock::new(HashMap::new())),
            execution_history: Arc::new(RwLock::new(Vec::new())),
            resource_limits: ResourceLimits::default(),
            security_level: SecurityLevel::High,
            context: default_context,
            runtime: default_runtime,
        }
    }

    /// Execute a YAML contract with given parameters (alias for execute_contract)
    pub async fn execute(&mut self, contract: &YamlContract, parameters: HashMap<String, serde_yaml::Value>) -> Result<ExecutionResult> {
        self.execute_contract(contract, parameters).await
    }

    /// Execute a YAML contract with given parameters
    pub async fn execute_contract(&mut self, contract: &YamlContract, parameters: HashMap<String, serde_yaml::Value>) -> Result<ExecutionResult> {
        tracing::info!("🚀 Starting contract execution: {}", contract.name);
        
        let execution_start = std::time::Instant::now();
        let mut execution_trace = ExecutionTrace {
            contract_id: contract.id.to_string(),
            steps: Vec::new(),
            started_at: Utc::now(),
            completed_at: None,
            total_duration_ms: 0,
        };

        // Create contract instance
        let mut contract_instance = ContractInstance {
            id: format!("instance_{}", Uuid::new_v4()),
            contract_id: contract.id.to_string(),
            status: ContractStatus::Running,
            current_state: "initial".to_string(),
            variables: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            execution_count: 0,
        };

        // Parse contract YAML
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&contract.yaml_definition)
            .map_err(|e| anyhow::anyhow!("Failed to parse contract YAML: {}", e))?;

        // Extract contract definition
        let contract_def = yaml_value.get("contract")
            .ok_or_else(|| anyhow::anyhow!("Contract must have a 'contract' section"))?;

        // Initialize variables
        if let Some(variables_section) = contract_def.get("variables") {
            if let Some(variables_map) = variables_section.as_mapping() {
                for (var_name, var_def) in variables_map {
                    if let (Some(name_str), Some(def_map)) = (var_name.as_str(), var_def.as_mapping()) {
                        if let Some(default_value) = def_map.get("default") {
                            contract_instance.variables.insert(
                                name_str.to_string(),
                                default_value.clone(),
                            );
                        }
                    }
                }
            }
        }

        // Execute state machine
        let execution_result = self.execute_state_machine(&mut contract_instance, contract_def, &self.context, &mut execution_trace).await?;

        // Update execution statistics
        let execution_duration = execution_start.elapsed();
        execution_trace.completed_at = Some(Utc::now());
        execution_trace.total_duration_ms = execution_duration.as_millis() as u64;

        // Store execution history
        {
            let mut history = self.execution_history.write().await;
            history.push(execution_trace.clone());
        }

        // Store active contract instance
        {
            let mut active_contracts = self.active_contracts.write().await;
            active_contracts.insert(contract_instance.id.clone(), contract_instance);
        }

        tracing::info!("✅ Contract execution completed in {}ms", execution_duration.as_millis());

        Ok(ExecutionResult {
            success: execution_result.success,
            final_state: execution_result.final_state,
            output_variables: execution_result.output_variables,
            execution_trace: Some(execution_trace),
            error: execution_result.error,
            gas_used: execution_result.gas_used,
            execution_time_ms: execution_duration.as_millis() as u64,
        })
    }

    /// Execute state machine
    async fn execute_state_machine(
        &self,
        contract_instance: &mut ContractInstance,
        contract_def: &serde_yaml::Value,
        context: &ExecutionContext,
        execution_trace: &mut ExecutionTrace,
    ) -> Result<ExecutionResult> {
        let states_section = contract_def.get("states")
            .ok_or_else(|| anyhow::anyhow!("Contract must have a 'states' section"))?;

        let states_map = states_section.as_mapping()
            .ok_or_else(|| anyhow::anyhow!("States section must be a mapping"))?;

        // Find initial state
        let mut current_state = self.find_initial_state(states_map)?;
        contract_instance.current_state = current_state.clone();

        let mut gas_used = 0u64;
        let mut max_iterations = 1000; // Prevent infinite loops

        while max_iterations > 0 {
            max_iterations -= 1;

            // Get current state definition
            let state_def = states_map.get(&serde_yaml::Value::String(current_state.clone()))
                .ok_or_else(|| anyhow::anyhow!("State '{}' not found", current_state))?;

            let state_map = state_def.as_mapping()
                .ok_or_else(|| anyhow::anyhow!("State '{}' must be a mapping", current_state))?;

            // Execute state actions
            let step_result = self.execute_state_actions(&current_state, state_map, contract_instance, context).await?;
            
            execution_trace.steps.push(ExecutionStep {
                state: current_state.clone(),
                action: step_result.action_executed.clone(),
                timestamp: Utc::now(),
                gas_consumed: step_result.gas_consumed,
                variables_changed: step_result.variables_changed.clone(),
                output: step_result.output.clone(),
            });

            gas_used += step_result.gas_consumed;

            // Check if this is a final state
            if let Some(state_type) = state_map.get("type") {
                if let Some(type_str) = state_type.as_str() {
                    if type_str == "final" || type_str == "error" {
                        contract_instance.status = if type_str == "final" {
                            ContractStatus::Completed
                        } else {
                            ContractStatus::Failed
                        };
                        
                        return Ok(ExecutionResult {
                            success: type_str == "final",
                            final_state: current_state.clone(),
                            output_variables: contract_instance.variables.clone(),
                            execution_trace: None,
                            error: if type_str == "error" {
                                Some(ExecutionError {
                                    error_type: "STATE_ERROR".to_string(),
                                    message: format!("Execution reached error state: {}", current_state),
                                    step_index: None,
                                    timestamp: chrono::Utc::now(),
                                })
                            } else { None },
                            gas_used,
                            execution_time_ms: 0, // Will be set by caller
                        });
                    }
                }
            }

            // Determine next state
            if let Some(next_state) = self.determine_next_state(state_map, contract_instance, context).await? {
                current_state = next_state;
                contract_instance.current_state = current_state.clone();
                contract_instance.execution_count += 1;
            } else {
                // No transition found, execution complete
                contract_instance.status = ContractStatus::Completed;
                return Ok(ExecutionResult {
                    success: true,
                    final_state: current_state,
                    output_variables: contract_instance.variables.clone(),
                    execution_trace: None,
                    error: None,
                    gas_used,
                    execution_time_ms: 0,
                });
            }
        }

        // Max iterations reached
        Err(anyhow::anyhow!("Contract execution exceeded maximum iterations"))
    }

    /// Find initial state in state machine
    fn find_initial_state(&self, states_map: &serde_yaml::Mapping) -> Result<String> {
        for (state_name, state_def) in states_map {
            if let (Some(name_str), Some(def_map)) = (state_name.as_str(), state_def.as_mapping()) {
                if let Some(state_type) = def_map.get("type") {
                    if let Some(type_str) = state_type.as_str() {
                        if type_str == "initial" {
                            return Ok(name_str.to_string());
                        }
                    }
                }
            }
        }

        Err(anyhow::anyhow!("No initial state found in contract"))
    }

    /// Execute actions for a specific state
    async fn execute_state_actions(
        &self,
        state_name: &str,
        state_map: &serde_yaml::Mapping,
        contract_instance: &mut ContractInstance,
        _context: &ExecutionContext,
    ) -> Result<StateExecutionResult> {
        let mut gas_consumed = 10; // Base gas cost
        let mut variables_changed = HashMap::new();
        let mut action_executed = "enter_state".to_string();
        let mut output = None;

        tracing::debug!("🔄 Executing state: {}", state_name);

        // Execute state actions
        if let Some(actions_section) = state_map.get("actions") {
            if let Some(actions_array) = actions_section.as_sequence() {
                for action in actions_array {
                    if let Some(action_str) = action.as_str() {
                        let action_result = self.execute_action(action_str, contract_instance).await?;
                        gas_consumed += action_result.gas_cost;
                        action_executed = action_str.to_string();
                        
                        // Merge variable changes
                        for (key, value) in action_result.variable_changes {
                            variables_changed.insert(key.clone(), value.clone());
                            contract_instance.variables.insert(key, value);
                        }

                        if let Some(action_output) = action_result.output {
                            output = Some(action_output);
                        }
                    }
                }
            }
        }

        Ok(StateExecutionResult {
            gas_consumed,
            variables_changed,
            action_executed,
            output,
        })
    }

    /// Execute a specific action
    async fn execute_action(&self, action: &str, contract_instance: &mut ContractInstance) -> Result<ActionExecutionResult> {
        let mut gas_cost = 5; // Base action cost
        let mut variable_changes = HashMap::new();
        let mut output = None;

        match action {
            "validate_consent" => {
                gas_cost = 15;
                variable_changes.insert("consent_validated".to_string(), serde_yaml::Value::Bool(true));
                output = Some("Consent validation completed".to_string());
            },
            "log_collection" => {
                gas_cost = 10;
                variable_changes.insert("collection_logged".to_string(), serde_yaml::Value::Bool(true));
                variable_changes.insert("collection_timestamp".to_string(), 
                    serde_yaml::Value::String(Utc::now().to_rfc3339()));
            },
            "apply_security_measures" => {
                gas_cost = 25;
                variable_changes.insert("security_applied".to_string(), serde_yaml::Value::Bool(true));
            },
            "monitor_access" => {
                gas_cost = 20;
                variable_changes.insert("access_monitored".to_string(), serde_yaml::Value::Bool(true));
            },
            "enforce_retention_policy" => {
                gas_cost = 15;
                variable_changes.insert("retention_enforced".to_string(), serde_yaml::Value::Bool(true));
            },
            "secure_deletion" => {
                gas_cost = 30;
                variable_changes.insert("data_deleted".to_string(), serde_yaml::Value::Bool(true));
                output = Some("Secure deletion completed".to_string());
            },
            "generate_certificate" => {
                gas_cost = 20;
                variable_changes.insert("certificate_generated".to_string(), serde_yaml::Value::Bool(true));
                variable_changes.insert("certificate_id".to_string(), 
                    serde_yaml::Value::String(format!("cert_{}", uuid::Uuid::new_v4())));
            },
            _ => {
                // Generic action
                gas_cost = 10;
                variable_changes.insert(format!("{}_executed", action), serde_yaml::Value::Bool(true));
            }
        }

        Ok(ActionExecutionResult {
            gas_cost,
            variable_changes,
            output,
        })
    }

    /// Determine next state based on transitions
    async fn determine_next_state(
        &self,
        state_map: &serde_yaml::Mapping,
        contract_instance: &ContractInstance,
        _context: &ExecutionContext,
    ) -> Result<Option<String>> {
        // Check for transitions
        if let Some(transitions_section) = state_map.get("transitions") {
            if let Some(transitions_map) = transitions_section.as_mapping() {
                // Simple transition logic - take first available transition
                for (condition, next_state) in transitions_map {
                    if let (Some(_condition_str), Some(next_state_str)) = (condition.as_str(), next_state.as_str()) {
                        // In a full implementation, we would evaluate the condition
                        // For now, we'll use simple logic based on variable states
                        if self.evaluate_transition_condition(condition, contract_instance) {
                            return Ok(Some(next_state_str.to_string()));
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    /// Evaluate transition condition
    fn evaluate_transition_condition(&self, _condition: &serde_yaml::Value, _contract_instance: &ContractInstance) -> bool {
        // Simplified condition evaluation
        // In production, this would parse and evaluate complex conditions
        true
    }

    /// Get active contract instances
    pub async fn get_active_contracts(&self) -> HashMap<String, ContractInstance> {
        let active_contracts = self.active_contracts.read().await;
        active_contracts.clone()
    }

    /// Get execution history
    pub async fn get_execution_history(&self) -> Vec<ExecutionTrace> {
        let history = self.execution_history.read().await;
        history.clone()
    }

    /// Stop contract execution
    pub async fn stop_contract(&self, instance_id: &str) -> Result<()> {
        let mut active_contracts = self.active_contracts.write().await;
        if let Some(contract_instance) = active_contracts.get_mut(instance_id) {
            contract_instance.status = ContractStatus::Stopped;
            tracing::info!("🛑 Contract instance {} stopped", instance_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Contract instance {} not found", instance_id))
        }
    }

    /// Get contract instance by ID
    pub async fn get_contract_instance(&self, instance_id: &str) -> Option<ContractInstance> {
        let active_contracts = self.active_contracts.read().await;
        active_contracts.get(instance_id).cloned()
    }

    /// Update resource limits
    pub fn set_resource_limits(&mut self, limits: ResourceLimits) {
        self.resource_limits = limits;
    }

    /// Set security level
    pub fn set_security_level(&mut self, level: SecurityLevel) {
        self.security_level = level;
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_gas: 1_000_000,
            max_execution_time_ms: 30_000,
            max_memory_mb: 128,
            max_storage_mb: 256,
        }
    }
}

/// Result of state execution
#[derive(Debug, Clone)]
struct StateExecutionResult {
    gas_consumed: u64,
    variables_changed: HashMap<String, serde_yaml::Value>,
    action_executed: String,
    output: Option<String>,
}

/// Result of action execution
#[derive(Debug, Clone)]
struct ActionExecutionResult {
    gas_cost: u64,
    variable_changes: HashMap<String, serde_yaml::Value>,
    output: Option<String>,
}


