# CollapseChip UPU v2.1 - Breadboard Schematic & Wiring Guide

## Complete Breadboard Layout

```
                    CollapseChip UPU v2.1 Breadboard Layout
    ┌─────────────────────────────────────────────────────────────────────────┐
    │  Power Rails: Red=+5V, Blue=+3.3V, Black=GND                          │
    ├─────────────────────────────────────────────────────────────────────────┤
    │                                                                         │
    │  [18650×2] → [TP4056] → [AMS1117-5V] → [AMS1117-3.3V]                │
    │     7.4V        5V          5V             3.3V                        │
    │                                                                         │
    ├─────────────────────────────────────────────────────────────────────────┤
    │  STM32F103C8T6 (Blue Pill)           RP2040 (Pico)                     │
    │  ┌─────────────────────────┐         ┌─────────────────────────┐       │
    │  │ PA0  ADC0          3.3V │         │ GP0  PWM0          VBUS │       │
    │  │ PA1  ADC1          GND  │         │ GP1  PWM1          VSYS │       │
    │  │ PA2  USART2_TX     5V   │         │ GP2  SPI0_SCK      GND  │       │
    │  │ PA3  USART2_RX     PB12 │         │ GP3  SPI0_TX       3V3  │       │
    │  │ PA4  SPI1_NSS      PB13 │         │ GP4  SPI0_RX       ADC  │       │
    │  │ PA5  SPI1_SCK      PB14 │         │ GP5  SPI0_CSN      GP28 │       │
    │  │ PA6  SPI1_MISO     PB15 │         │ GP6  I2C1_SDA      GND  │       │
    │  │ PA7  SPI1_MOSI     PA8  │         │ GP7  I2C1_SCL      GP27 │       │
    │  │ PB0  ADC8          PA9  │         │ GP8  UART1_TX      GP26 │       │
    │  │ PB1  ADC9          PA10 │         │ GP9  UART1_RX      RUN  │       │
    │  │ PB6  I2C1_SCL      PA11 │         │ GP10 PWM5A         GP22 │       │
    │  │ PB7  I2C1_SDA      PA12 │         │ GP11 PWM5B         GND  │       │
    │  │ PB8  TIM4_CH3      PA15 │         │ GP12 PWM6A         GP21 │       │
    │  │ PB9  TIM4_CH4      PB3  │         │ GP13 PWM6B         GP20 │       │
    │  │ PB10 I2C2_SCL      PB4  │         │ GP14 PWM7A         GP19 │       │
    │  │ PB11 I2C2_SDA      PB5  │         │ GP15 PWM7B         GP18 │       │
    │  └─────────────────────────┘         └─────────────────────────┘       │
    │                                                                         │
    ├─────────────────────────────────────────────────────────────────────────┤
    │  ESP32-WROOM-32                      Analog Processing                  │
    │  ┌─────────────────────────┐         ┌─────────────────────────┐       │
    │  │ EN   3V3          GND   │         │ MCP4725 DAC    ADS1115  │       │
    │  │ VP   GPIO36       GPIO23│         │ VCC → 3.3V     VCC → 3.3V│      │
    │  │ VN   GPIO39       GPIO22│         │ GND → GND      GND → GND │       │
    │  │ GPIO34 GPIO35     TXD0  │         │ SCL → PB6      SCL → PB6 │       │
    │  │ GPIO32 GPIO33     RXD0  │         │ SDA → PB7      SDA → PB7 │       │
    │  │ GPIO25 GPIO26     GPIO21│         │ OUT → ADS A0   A0-A3 → Analog│   │
    │  │ GPIO27 GPIO14     GND   │         └─────────────────────────┘       │
    │  │ GPIO12 GPIO13     GPIO19│                                           │
    │  │ GND    GPIO15     GPIO18│         2N7000 MOSFET Array (10×)         │
    │  │ GPIO2  GPIO4      GPIO5 │         ┌─────────────────────────┐       │
    │  │ GPIO0  GPIO16     GPIO17│         │ G1→GP10  D1→Analog_Out1 │       │
    │  │ 3V3    GPIO17     GND   │         │ G2→GP11  D2→Analog_Out2 │       │
    │  └─────────────────────────┘         │ G3→GP12  D3→Analog_Out3 │       │
    │                                      │ G4→GP13  D4→Analog_Out4 │       │
    ├─────────────────────────────────────────────────────────────────────────┤
    │  Display Modules                     Memory Modules                     │
    │  ┌─────────────────────────┐         ┌─────────────────────────┐       │
    │  │ ST7735 1.8" TFT         │         │ AT24C256 EEPROM         │       │
    │  │ VCC → 3.3V              │         │ VCC → 3.3V              │       │
    │  │ GND → GND               │         │ GND → GND               │       │
    │  │ SCL → GP2 (SPI0_SCK)    │         │ SCL → PB6 (I2C1_SCL)    │       │
    │  │ SDA → GP3 (SPI0_TX)     │         │ SDA → PB7 (I2C1_SDA)    │       │
    │  │ RES → GP4               │         │ A0,A1,A2 → GND          │       │
    │  │ DC  → GP5               │         └─────────────────────────┘       │
    │  │ CS  → GP6               │                                           │
    │  │ BLK → GP7               │         MicroSD Card Module               │
    │  └─────────────────────────┘         ┌─────────────────────────┐       │
    │                                      │ VCC → 5V                │       │
    │  SSD1306 OLED 128×64                │ GND → GND               │       │
    │  ┌─────────────────────────┐         │ MISO → PA6 (SPI1_MISO) │       │
    │  │ VCC → 3.3V              │         │ MOSI → PA7 (SPI1_MOSI) │       │
    │  │ GND → GND               │         │ SCK → PA5 (SPI1_SCK)   │       │
    │  │ SCL → PB6 (I2C1_SCL)    │         │ CS → PA4 (SPI1_NSS)    │       │
    │  │ SDA → PB7 (I2C1_SDA)    │         └─────────────────────────┘       │
    │  └─────────────────────────┘                                           │
    └─────────────────────────────────────────────────────────────────────────┘
```

## Detailed Wiring Instructions

### Power Distribution Network
```
Battery Pack (2× 18650 Li-ion in series = 7.4V)
    ↓
TP4056 Charging Module
    ├── BAT+ → 18650 Positive Terminal
    ├── BAT- → 18650 Negative Terminal  
    ├── OUT+ → AMS1117-5V Input
    └── OUT- → Common Ground

AMS1117-5V Regulator
    ├── Input: 7.4V from TP4056
    ├── Output: 5V → Red Power Rail
    └── Ground → Black Ground Rail

AMS1117-3.3V Regulator  
    ├── Input: 5V from first regulator
    ├── Output: 3.3V → Blue Power Rail
    └── Ground → Black Ground Rail
```

### Inter-MCU Communication (SPI Bridge)
```
STM32F103 (Master) ↔ RP2040 (Slave)
PA5 (SPI1_SCK)  → GP2 (SPI0_SCK)
PA6 (SPI1_MISO) ← GP4 (SPI0_RX) 
PA7 (SPI1_MOSI) → GP3 (SPI0_TX)
PA4 (SPI1_NSS)  → GP5 (SPI0_CSN)
```

### I2C Bus Connections
```
I2C Bus (STM32F103 as Master):
PB6 (SCL) → Pull-up 4.7kΩ to 3.3V → All I2C Device SCL pins
PB7 (SDA) → Pull-up 4.7kΩ to 3.3V → All I2C Device SDA pins

Connected I2C Devices:
├── MCP4725 DAC (Address: 0x60)
├── ADS1115 ADC (Address: 0x48) 
├── AT24C256 EEPROM (Address: 0x50)
└── SSD1306 OLED (Address: 0x3C)
```

### Analog Processing Network
```
Symbolic Gate Array (2N7000 MOSFETs):
Gate Control (RP2040) → MOSFET Gates → Analog Outputs
GP10 → 1kΩ → 2N7000_1 Gate → Drain to Analog_Out_1
GP11 → 1kΩ → 2N7000_2 Gate → Drain to Analog_Out_2
GP12 → 1kΩ → 2N7000_3 Gate → Drain to Analog_Out_3
GP13 → 1kΩ → 2N7000_4 Gate → Drain to Analog_Out_4
(Continue for all 10 MOSFETs using GP14-GP19)

Op-Amp Signal Conditioning:
LM358 Pin 1 (OUT1) → ADS1115 A0 (Entropy Measurement)
LM358 Pin 2 (IN1-) → MCP4725 OUT (DAC Feedback)
LM358 Pin 3 (IN1+) → Analog_Out_1 (MOSFET Array)
LM358 Pin 7 (OUT2) → ADS1115 A1 (Signal Conditioning)
LM358 Pin 6 (IN2-) → Reference Voltage (1.65V)
LM358 Pin 5 (IN2+) → Analog_Out_2 (MOSFET Array)
```

### Display Connections
```
ST7735 TFT Display (Primary Graphics):
VCC → 3.3V Rail
GND → Ground Rail  
SCL → GP2 (RP2040 SPI0_SCK)
SDA → GP3 (RP2040 SPI0_TX)
RES → GP4 (Reset control)
DC  → GP5 (Data/Command select)
CS  → GP6 (Chip select)
BLK → GP7 (Backlight control)

SSD1306 OLED Display (Status):
VCC → 3.3V Rail
GND → Ground Rail
SCL → PB6 (STM32 I2C1_SCL)
SDA → PB7 (STM32 I2C1_SDA)
```

### User Interface
```
Rotary Encoder with Push Button:
CLK → PA0 (STM32 GPIO with interrupt)
DT  → PA1 (STM32 GPIO with interrupt)  
SW  → PA2 (STM32 GPIO with interrupt)
VCC → 3.3V Rail
GND → Ground Rail

4×4 Matrix Keypad:
Row pins → PA8, PA9, PA10, PA11 (STM32 GPIO outputs)
Col pins → PA12, PA15, PB3, PB4 (STM32 GPIO inputs with pull-ups)
```

## Component Placement Guide

### Breadboard Layout (830-point breadboard)
```
Top Section (Rows 1-10):
├── Power regulation circuit (TP4056, AMS1117s)
├── STM32F103C8T6 Blue Pill (centered)
└── Power distribution rails

Middle Section (Rows 11-25):
├── RP2040 Pico (left side)
├── ESP32-WROOM-32 (right side)  
├── I2C devices (MCP4725, ADS1115, EEPROM)
└── SPI bridge connections

Bottom Section (Rows 26-30):
├── 2N7000 MOSFET array
├── LM358 op-amp circuits
├── Display modules
└── User interface components
```

### Wire Management
- **Red wires**: +5V power connections
- **Blue wires**: +3.3V power connections  
- **Black wires**: Ground connections
- **Yellow wires**: SPI communications
- **Green wires**: I2C communications
- **White wires**: GPIO and analog signals

## Testing and Verification

### Power System Test
```c
// Test power rails and current consumption
void test_power_system() {
    // Measure voltages with multimeter
    printf("5V Rail: %.2fV\n", measure_voltage(5V_RAIL));
    printf("3.3V Rail: %.2fV\n", measure_voltage(3V3_RAIL));
    printf("Current Draw: %.1fmA\n", measure_current());
    
    // Test battery charging
    printf("Battery Voltage: %.2fV\n", measure_battery());
    printf("Charging Status: %s\n", get_charge_status());
}
```

### Communication Test
```c
// Test SPI bridge between STM32 and RP2040
void test_spi_bridge() {
    uint8_t test_data[] = {0xAA, 0x55, 0xFF, 0x00};
    uint8_t received[4];
    
    spi_transmit_receive(test_data, received, 4);
    
    for(int i = 0; i < 4; i++) {
        printf("Sent: 0x%02X, Received: 0x%02X\n", 
               test_data[i], received[i]);
    }
}

// Test I2C bus communication
void test_i2c_devices() {
    uint8_t devices[] = {0x60, 0x48, 0x50, 0x3C}; // DAC, ADC, EEPROM, OLED
    char* names[] = {"MCP4725", "ADS1115", "AT24C256", "SSD1306"};
    
    for(int i = 0; i < 4; i++) {
        if(i2c_device_present(devices[i])) {
            printf("%s detected at 0x%02X\n", names[i], devices[i]);
        } else {
            printf("%s NOT FOUND at 0x%02X\n", names[i], devices[i]);
        }
    }
}
```

### Analog System Test
```c
// Test DAC output and ADC input
void test_analog_system() {
    // Test DAC output sweep
    for(uint16_t value = 0; value < 4096; value += 256) {
        mcp4725_write(value);
        delay_ms(100);
        
        uint16_t adc_reading = ads1115_read(0);
        float voltage = (adc_reading * 3.3f) / 4096.0f;
        
        printf("DAC: %d, ADC: %d, Voltage: %.3fV\n", 
               value, adc_reading, voltage);
    }
}

// Test MOSFET gate array
void test_mosfet_array() {
    for(int gate = 0; gate < 10; gate++) {
        // Turn on gate
        gpio_set(MOSFET_GATES[gate]);
        delay_ms(100);
        
        // Measure output
        uint16_t output = ads1115_read(gate % 4);
        printf("Gate %d ON: ADC = %d\n", gate, output);
        
        // Turn off gate  
        gpio_clear(MOSFET_GATES[gate]);
        delay_ms(100);
        
        output = ads1115_read(gate % 4);
        printf("Gate %d OFF: ADC = %d\n", gate, output);
    }
}
```

### Display Test
```c
// Test TFT display
void test_tft_display() {
    st7735_init();
    st7735_fill_screen(COLOR_BLACK);
    
    // Draw test pattern
    st7735_draw_rectangle(10, 10, 50, 30, COLOR_RED);
    st7735_draw_circle(80, 40, 20, COLOR_GREEN);
    st7735_draw_line(0, 0, 159, 127, COLOR_BLUE);
    
    // Display text
    st7735_draw_string(10, 60, "CollapseChip UPU v2.1", COLOR_WHITE);
    st7735_draw_string(10, 80, "Symbolic Processing", COLOR_YELLOW);
}

// Test OLED display  
void test_oled_display() {
    ssd1306_init();
    ssd1306_clear();
    
    ssd1306_draw_string(0, 0, "System Status:");
    ssd1306_draw_string(0, 16, "CPU: STM32F103");
    ssd1306_draw_string(0, 32, "GPU: RP2040");
    ssd1306_draw_string(0, 48, "Status: Online");
    
    ssd1306_update();
}
```

## Troubleshooting Guide

### Common Issues and Solutions

#### Power Problems
**Symptom**: No power to MCUs
- Check battery voltage (should be 6.0-8.4V)
- Verify TP4056 output (should be ~7.4V)
- Check AMS1117 regulators (5V and 3.3V outputs)
- Ensure proper ground connections

**Symptom**: High current consumption
- Disconnect ESP32 (WiFi can draw significant current)
- Check for short circuits in analog section
- Verify MOSFET gates are properly controlled

#### Communication Issues
**Symptom**: SPI bridge not working
- Verify SPI pin connections (SCK, MISO, MOSI, CS)
- Check logic levels with oscilloscope
- Ensure proper SPI configuration (mode, speed)
- Add pull-up resistors if needed

**Symptom**: I2C devices not responding
- Check pull-up resistors on SCL/SDA (4.7kΩ to 3.3V)
- Verify device addresses with I2C scanner
- Check for bus conflicts or multiple masters
- Ensure proper power to all I2C devices

#### Display Problems
**Symptom**: TFT display blank
- Check SPI connections and chip select
- Verify backlight connection and control
- Check display initialization sequence
- Measure power supply to display

**Symptom**: OLED display not updating
- Verify I2C communication to 0x3C address
- Check display initialization
- Ensure proper contrast and display enable
- Test with simple pixel patterns

#### Analog Issues
**Symptom**: DAC output incorrect
- Check I2C communication to MCP4725
- Verify reference voltage and power supply
- Test with known values and multimeter
- Check for loading effects

**Symptom**: ADC readings unstable
- Add filtering capacitors to analog inputs
- Check reference voltage stability
- Ensure proper grounding and shielding
- Use differential measurements if possible

## Safety Considerations

### Electrical Safety
- Always disconnect power when making connections
- Use proper ESD precautions when handling components
- Double-check polarity before applying power
- Use current-limited power supply during testing

### Component Protection
- Add fuses to power inputs (1A for main supply)
- Use TVS diodes on exposed I/O pins
- Implement software watchdog timers
- Add reverse polarity protection

### Thermal Management
- Monitor component temperatures during operation
- Ensure adequate ventilation around regulators
- Add heatsinks to high-power components if needed
- Implement thermal shutdown in software

## Performance Optimization

### Power Optimization
```c
// Enable low-power modes when idle
void optimize_power() {
    // STM32 sleep mode
    HAL_PWR_EnterSLEEPMode(PWR_MAINREGULATOR_ON, PWR_SLEEPENTRY_WFI);
    
    // RP2040 dormant mode
    sleep_goto_dormant_until_pin(WAKEUP_PIN, true, false);
    
    // ESP32 light sleep
    esp_sleep_enable_timer_wakeup(1000000); // 1 second
    esp_light_sleep_start();
}
```

### Memory Optimization
```c
// Use efficient data structures
typedef struct __attribute__((packed)) {
    uint16_t state : 2;      // Morphon state (2 bits)
    uint16_t entropy : 10;   // Entropy value (10 bits)  
    uint16_t transition : 4; // Transition number (4 bits)
} CompactMorphon;

// Pool memory allocation
#define MORPHON_POOL_SIZE 1000
CompactMorphon morphon_pool[MORPHON_POOL_SIZE];
uint16_t pool_index = 0;
```

### Processing Optimization
```c
// Use hardware acceleration where available
void optimize_processing() {
    // Enable STM32 DMA for SPI transfers
    HAL_SPI_Transmit_DMA(&hspi1, data, length);
    
    // Use RP2040 PIO for parallel processing
    pio_sm_config config = pio_get_default_sm_config();
    pio_sm_init(pio0, sm, offset, &config);
    
    // Optimize floating-point operations
    arm_sin_f32(angle, &result); // Use CMSIS DSP library
}
```

This comprehensive breadboard schematic and wiring guide provides everything needed to build the CollapseChip UPU v2.1 prototype. The detailed instructions, testing procedures, and troubleshooting guide ensure successful assembly and operation of this revolutionary symbolic processing system.
