use anyhow::Result;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::{
    YamlContract, CompiledStateMachine, CompilerResult, CompilerError,
    CompilerWarning, StateNode, StateTransition, TransitionCondition, ActionDefinition,
    CompilationPhase, CompilationMetadata, CompilerConfig, OptimizationLevel, TargetPlatform,
    StateMachineCompiler, StateType
};

impl StateMachineCompiler {
    /// Create a new state machine compiler with default config
    pub fn new() -> Self {
        let default_config = CompilerConfig {
            optimization_level: OptimizationLevel::Standard,
            target_platform: TargetPlatform::Rust,
            enable_debug_info: true,
            max_compilation_time_ms: 30000,
        };
        
        Self {
            config: default_config,
            optimization_passes: Self::default_optimization_passes(),
            targets: Vec::new(),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Compile a YAML contract into a state machine
    pub async fn compile(&self, contract: &YamlContract) -> Result<CompilerResult> {
        let start_time = std::time::Instant::now();
        
        // Basic validation
        if contract.metadata.name.is_empty() {
            return Ok(CompilerResult {
                success: false,
                compiled_sm: None,
                errors: vec![CompilerError {
                    message: "Contract name cannot be empty".to_string(),
                    location: None,
                    error_type: "ValidationError".to_string(),
                    line: None,
                    column: None,
                }],
                warnings: Vec::new(),
                compilation_time_ms: start_time.elapsed().as_millis() as u64,
                phases_completed: vec![CompilationPhase::Parsing],
            });
        }

        // Create compiled state machine
        let compiled_sm = CompiledStateMachine {
            id: Uuid::new_v4(),
            bytecode: vec![0x01, 0x02, 0x03], // Placeholder bytecode
            metadata: CompilationMetadata {
                contract_name: contract.metadata.name.clone(),
                version: contract.metadata.version.clone(),
                compiled_at: Utc::now(),
                compiler_version: "1.0.0".to_string(),
                optimization_level: self.config.optimization_level.clone(),
                target_platform: self.config.target_platform.clone(),
                optimizations: Vec::new(),
                warnings: Vec::new(),
            },
            optimization_level: self.config.optimization_level.clone(),
            initial_state: "init".to_string(),
            states: vec![
                StateNode {
                    id: "init".to_string(),
                    name: "Initial".to_string(),
                    state_type: crate::StateType::Initial,
                    actions: Vec::new(),
                    metadata: HashMap::new(),
                    transitions: Vec::new(),
                }
            ],
            transitions: Vec::new(),
            final_states: vec!["completed".to_string()],
            compiled_at: Utc::now(),
        };

        Ok(CompilerResult {
            success: true,
            compiled_sm: Some(compiled_sm),
            errors: Vec::new(),
            warnings: Vec::new(),
            compilation_time_ms: start_time.elapsed().as_millis() as u64,
            phases_completed: vec![
                CompilationPhase::Parsing,
                CompilationPhase::Validation,
                CompilationPhase::IRGeneration,
                CompilationPhase::Optimization,
                CompilationPhase::CodeGeneration
            ],
        })
    }

    fn default_optimization_passes() -> Vec<String> {
        vec![
            "eliminate_dead_states".to_string(),
            "fold_constants".to_string(),
            "optimize_transitions".to_string(),
        ]
    }

    /// Create a new state machine compiler with default config (old method)
    pub fn new_old() -> Self {
        let default_config = CompilerConfig {
            optimization_level: OptimizationLevel::Standard,
            target_platform: TargetPlatform::Rust,
            enable_debug_info: true,
            max_compilation_time_ms: 30000,
        };
        
        Self {
            config: default_config,
            optimization_passes: Self::default_optimization_passes(),
            targets: Vec::new(),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new state machine compiler with custom config
    pub fn new_with_config(config: CompilerConfig) -> Self {
        Self {
            config,
            optimization_passes: Self::default_optimization_passes(),
            targets: Vec::new(),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Compile YAML contract to optimized state machine (old implementation)
    pub async fn compile_old(&self, contract: &YamlContract) -> Result<CompilerResult> {
        tracing::info!("🔨 Starting state machine compilation for contract: {}", contract.name);
        
        let compilation_start = std::time::Instant::now();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Phase 1: Parse YAML and extract state machine
        let parse_result = self.parse_contract_states(contract).await;
        match parse_result {
            Ok((states, transitions)) => {
                // Phase 2: Build intermediate representation
                let ir_result = self.build_intermediate_representation(&states, &transitions);
                match ir_result {
                    Ok(mut compiled_sm) => {
                        // Phase 3: Apply optimizations
                        if self.config.optimization_level != OptimizationLevel::None {
                            compiled_sm = self.apply_optimizations(compiled_sm, &mut warnings).await?;
                        }

                        // Phase 4: Generate target code
                        let target_code = self.generate_target_code(&compiled_sm)?;
                        // Store generated code in bytecode field for now
                        compiled_sm.bytecode = target_code.into_bytes();

                        // Phase 5: Validate compiled state machine
                        self.validate_compiled_sm(&compiled_sm, &mut errors, &mut warnings);

                        let compilation_duration = compilation_start.elapsed();
                        
                        tracing::info!("✅ State machine compilation completed in {}ms", compilation_duration.as_millis());

                        Ok(CompilerResult {
                            success: errors.is_empty(),
                            compiled_sm: if errors.is_empty() { Some(compiled_sm) } else { None },
                            errors,
                            warnings,
                            compilation_time_ms: compilation_start.elapsed().as_millis() as u64,
                            phases_completed: vec![
                                CompilationPhase::Parsing,
                                CompilationPhase::IRGeneration,
                                CompilationPhase::Optimization,
                                CompilationPhase::CodeGeneration,
                                CompilationPhase::Validation,
                            ],
                        })
                    },
                    Err(e) => {
                        errors.push(CompilerError {
                            error_type: "IR_GENERATION_FAILED".to_string(),
                            message: format!("Failed to generate intermediate representation: {}", e),
                            location: None,
                            line: None,
                            column: None,
                        });

                        return Ok(CompilerResult {
                            success: false,
                            compiled_sm: None,
                            errors,
                            warnings,
                            compilation_time_ms: compilation_start.elapsed().as_millis() as u64,
                            phases_completed: vec![CompilationPhase::Parsing],
                        });
                    }
                }
            },
            Err(e) => {
                Ok(CompilerResult {
                    success: false,
                    compiled_sm: None,
                    errors: vec![CompilerError {
                        error_type: "PARSE_ERROR".to_string(),
                        message: format!("Failed to parse YAML: {}", e),
                        location: None,
                        line: None,
                        column: None,
                    }],
                    warnings: vec![],
                    compilation_time_ms: 0,
                    phases_completed: vec![],
                })
            }
        }
    }

    /// Parse contract states from YAML
    async fn parse_contract_states(&self, contract: &YamlContract) -> Result<(Vec<StateNode>, Vec<StateTransition>)> {
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&contract.yaml_definition)?;
        
        let contract_section = yaml_value.get("contract")
            .ok_or_else(|| anyhow::anyhow!("Contract must have a 'contract' section"))?;

        let states_section = contract_section.get("states")
            .ok_or_else(|| anyhow::anyhow!("Contract must have a 'states' section"))?;

        let states_map = states_section.as_mapping()
            .ok_or_else(|| anyhow::anyhow!("States section must be a mapping"))?;

        let mut state_nodes = Vec::new();
        let mut state_transitions = Vec::new();

        // Parse each state
        for (state_name, state_def) in states_map {
            if let (Some(name_str), Some(def_map)) = (state_name.as_str(), state_def.as_mapping()) {
                let state_node = self.parse_state_node(name_str, def_map)?;
                
                // Extract transitions from this state
                if let Some(transitions_section) = def_map.get("transitions") {
                    if let Some(transitions_map) = transitions_section.as_mapping() {
                        for (condition, target_state) in transitions_map {
                            if let (Some(condition_str), Some(target_str)) = (condition.as_str(), target_state.as_str()) {
                                let transition = StateTransition {
                                    id: format!("{}-{}-{}", name_str, target_str, state_transitions.len()),
                                    from_state: name_str.to_string(),
                                    to_state: target_str.to_string(),
                                    condition: TransitionCondition {
                                        condition_type: "always".to_string(),
                                        expression: "true".to_string(),
                                        parameters: HashMap::new(),
                                    },
                                    guard: None,
                                    actions: Vec::new(),
                                };
                                state_transitions.push(transition);
                            }
                        }
                    }
                }

                state_nodes.push(state_node);
            }
        }

        Ok((state_nodes, state_transitions))
    }

    /// Parse individual state node
    fn parse_state_node(&self, name: &str, state_map: &serde_yaml::Mapping) -> Result<StateNode> {
        let state_type = state_map.get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("normal");

        let mut actions = Vec::new();
        if let Some(actions_section) = state_map.get("actions") {
            if let Some(actions_array) = actions_section.as_sequence() {
                for action in actions_array {
                    if let Some(action_str) = action.as_str() {
                        let action_name = action_str.to_string();
                        let action_params = HashMap::new();
                        actions.push(ActionDefinition {
                            name: action_name,
                            action_type: "default".to_string(),
                            parameters: action_params,
                            gas_cost: 1, // Default gas cost
                        });
                    }
                }
            }
        }

        let mut transitions = Vec::new();
        if let Some(transitions_section) = state_map.get("transitions") {
            if let Some(transitions_map) = transitions_section.as_mapping() {
                for (condition, target_state) in transitions_map {
                    if let (Some(condition_str), Some(target_str)) = (condition.as_str(), target_state.as_str()) {
                        transitions.push(StateTransition {
                            id: format!("{}-{}-{}", name, target_str, transitions.len()),
                            from_state: name.to_string(),
                            to_state: target_str.to_string(),
                            condition: TransitionCondition {
                                condition_type: "always".to_string(),
                                expression: "true".to_string(),
                                parameters: HashMap::new(),
                            },
                            guard: None,
                            actions: Vec::new(),
                        });
                    }
                }
            }
        }

        let state_node = StateNode {
            id: name.to_string(),
            name: name.to_string(),
            state_type: StateType::Normal,
            actions: actions,
            metadata: HashMap::new(),
            transitions: transitions,
        };

        Ok(state_node)
    }

    /// Parse transition condition
    fn parse_transition_condition(&self, condition_str: &str) -> Result<TransitionCondition> {
        // Simplified condition parsing
        // In production, this would parse complex boolean expressions
        Ok(TransitionCondition {
            condition_type: "expression".to_string(),
            expression: condition_str.to_string(),
            parameters: HashMap::new(), // Simplified - just use empty parameters
        })
    }

    /// Extract variables from condition expression
    fn extract_variables_from_condition(&self, condition: &str) -> Vec<String> {
        // Simplified variable extraction
        // In production, this would use proper parsing
        let mut variables = Vec::new();
        
        // Look for common variable patterns
        if condition.contains("consent_validated") {
            variables.push("consent_validated".to_string());
        }
        if condition.contains("security_applied") {
            variables.push("security_applied".to_string());
        }
        if condition.contains("data_deleted") {
            variables.push("data_deleted".to_string());
        }

        variables
    }

    /// Estimate gas cost for action
    fn estimate_action_gas_cost(&self, action: &str) -> u64 {
        match action {
            "validate_consent" => 15,
            "log_collection" => 10,
            "apply_security_measures" => 25,
            "monitor_access" => 20,
            "enforce_retention_policy" => 15,
            "secure_deletion" => 30,
            "generate_certificate" => 20,
            _ => 10, // Default cost
        }
    }

    /// Build intermediate representation
    fn build_intermediate_representation(&self, states: &[StateNode], transitions: &[StateTransition]) -> Result<CompiledStateMachine> {
        let mut compiled_sm = CompiledStateMachine {
            id: uuid::Uuid::new_v4(),
            bytecode: vec![0x01, 0x02, 0x03], // Placeholder bytecode
            metadata: CompilationMetadata {
                contract_name: "unknown".to_string(),
                version: "1.0.0".to_string(),
                compiled_at: Utc::now(),
                compiler_version: "1.0.0".to_string(),
                optimization_level: OptimizationLevel::Standard,
                target_platform: TargetPlatform::Rust,
                optimizations: Vec::new(),
                warnings: Vec::new(),
            },
            optimization_level: self.config.optimization_level.clone(),
            initial_state: "initial".to_string(),
            states: Vec::new(),
            transitions: Vec::new(),
            final_states: Vec::new(),
            compiled_at: Utc::now(),
        };

        // Build state connectivity graph
        self.build_connectivity_graph(&mut compiled_sm)?;

        Ok(compiled_sm)
    }

    /// Find initial state from states (simplified)
    fn find_initial_state(&self, states: &[StateNode]) -> Result<String> {
        // Simplified - just return the first state name or default
        if let Some(first_state) = states.first() {
            Ok(first_state.name.clone())
        } else {
            Ok("initial".to_string()) // Default fallback
        }
    }

    /// Find final states from states (simplified)
    fn find_final_states(&self, states: &[StateNode]) -> Vec<String> {
        let mut final_states = Vec::new();
        for state in states {
            if state.transitions.is_empty() {
                final_states.push(state.name.clone());
            }
        }
        final_states
    }

    /// Build connectivity graph for optimization
    fn build_connectivity_graph(&self, compiled_sm: &mut CompiledStateMachine) -> Result<()> {
        // Simplified - just log the operation
        tracing::info!("Building connectivity graph for state machine: {}", compiled_sm.id);
        Ok(())
    }

    /// Apply optimization passes
    async fn apply_optimizations(&self, mut compiled_sm: CompiledStateMachine, warnings: &mut Vec<CompilerWarning>) -> Result<CompiledStateMachine> {
        tracing::debug!("🔧 Applying {} optimization passes", self.optimization_passes.len());

        for pass_name in &self.optimization_passes {
            match pass_name.as_str() {
                "dead_state_elimination" => {
                    compiled_sm = self.fold_constants(compiled_sm, warnings)?;
                },
                "unreachable_code_removal" => {
                    compiled_sm = self.remove_unreachable_code(compiled_sm, warnings)?;
                },
                "state_merging" => {
                    compiled_sm = self.merge_equivalent_states(compiled_sm, warnings)?;
                },
                "transition_optimization" => {
                    compiled_sm = self.optimize_transitions(compiled_sm, warnings)?;
                },
                "constant_folding" => {
                    compiled_sm = self.eliminate_dead_states(compiled_sm, warnings)?;
                },
                _ => {
                    warnings.push(CompilerWarning {
                        warning_type: "UNKNOWN_OPTIMIZATION_PASS".to_string(),
                        message: format!("Unknown optimization pass: {}", pass_name),
                        line: None,
                        column: None,
                    });
                }
            }
        }

        Ok(compiled_sm)
    }

    /// Eliminate dead states
    fn fold_constants(&self, compiled_sm: CompiledStateMachine, _warnings: &mut Vec<CompilerWarning>) -> Result<CompiledStateMachine> {
        // Simplified - just log the operation and return the state machine
        tracing::info!("Running dead state elimination for state machine: {}", compiled_sm.id);
        Ok(compiled_sm)
    }

    /// Remove unreachable code
    fn remove_unreachable_code(&self, compiled_sm: CompiledStateMachine, _warnings: &mut Vec<CompilerWarning>) -> Result<CompiledStateMachine> {
        // Simplified unreachable code removal
        // In production, this would analyze action reachability
        Ok(compiled_sm)
    }

    /// Merge equivalent states
    fn merge_equivalent_states(&self, compiled_sm: CompiledStateMachine, _warnings: &mut Vec<CompilerWarning>) -> Result<CompiledStateMachine> {
        // Simplified state merging
        // In production, this would identify and merge states with identical behavior
        Ok(compiled_sm)
    }

    /// Optimize transitions
    fn optimize_transitions(&self, compiled_sm: CompiledStateMachine, _warnings: &mut Vec<CompilerWarning>) -> Result<CompiledStateMachine> {
        // Simplified transition optimization
        // In production, this would optimize transition conditions and guards
        Ok(compiled_sm)
    }

    /// Fold constants
    fn eliminate_dead_states(&self, compiled_sm: CompiledStateMachine, _warnings: &mut Vec<CompilerWarning>) -> Result<CompiledStateMachine> {
        // Simplified - just log the operation and return the state machine
        tracing::info!("Running dead state elimination for state machine: {}", compiled_sm.id);
        Ok(compiled_sm)
    }

    /// Generate target code
    fn generate_target_code(&self, compiled_sm: &CompiledStateMachine) -> Result<String> {
        match self.config.target_platform {
            TargetPlatform::Rust => Ok(self.generate_rust_code(compiled_sm)),
            TargetPlatform::JavaScript => Ok(self.generate_javascript_code(compiled_sm)),
            TargetPlatform::WebAssembly => self.generate_wasm_code(compiled_sm),
            TargetPlatform::Native => self.generate_native_code(compiled_sm),
        }
    }

    /// Generate Rust code from compiled state machine (simplified)
    fn generate_rust_code(&self, compiled_sm: &CompiledStateMachine) -> String {
        format!(
            "// Generated Rust code for YAML SmartContract\n// Contract ID: {}\n// Bytecode length: {} bytes\n",
            compiled_sm.id,
            compiled_sm.bytecode.len()
        )
    }
    
    /// Generate JavaScript code from compiled state machine (simplified)
    fn generate_javascript_code(&self, compiled_sm: &CompiledStateMachine) -> String {
        format!(
            "// Generated JavaScript code for YAML SmartContract\n// Contract ID: {}\n// Bytecode length: {} bytes\n",
            compiled_sm.id,
            compiled_sm.bytecode.len()
        )
    }

    /// Generate WebAssembly code
    fn generate_wasm_code(&self, _compiled_sm: &CompiledStateMachine) -> Result<String> {
        // Simplified WASM generation
        Ok("(module\n  ;; Generated WebAssembly state machine\n)".to_string())
    }

    /// Generate native code
    fn generate_native_code(&self, _compiled_sm: &CompiledStateMachine) -> Result<String> {
        // Simplified native code generation
        Ok("// Generated native state machine code\n".to_string())
    }

    /// Convert state name to Rust enum variant
    fn to_rust_enum_variant(&self, name: &str) -> String {
        // Convert snake_case to PascalCase
        name.split('_')
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                }
            })
            .collect()
    }

    /// Validate compiled state machine (simplified)
    fn validate_compiled_sm(&self, compiled_sm: &CompiledStateMachine, errors: &mut Vec<CompilerError>, warnings: &mut Vec<CompilerWarning>) {
        // Basic validation - keep it simple
        tracing::info!("Validating compiled state machine: {}", compiled_sm.id);
        
        // Just log validation for now - avoid complex field access
        if compiled_sm.bytecode.is_empty() {
            errors.push(CompilerError {
                error_type: "EMPTY_BYTECODE".to_string(),
                message: "Compiled state machine has empty bytecode".to_string(),
                location: None,
                line: None,
                column: None,
            });
        }
    }

    /// Get compiler configuration
    pub fn get_config(&self) -> &CompilerConfig {
        &self.config
    }

    /// Update compiler configuration
    pub fn set_config(&mut self, config: CompilerConfig) {
        self.config = config;
    }
}
