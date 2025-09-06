# UPC v2.2 PRO - Physical Board Layout Guide

## 🎯 Beginner-Friendly Component Placement

This guide shows you **exactly where to place each component** on your breadboards. Follow the visual diagrams and you'll have a perfectly organized symbolic entropy computer!

---

## 📐 Overview: Three-Breadboard System

```
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│   BREADBOARD 1  │  │   BREADBOARD 2  │  │   BREADBOARD 3  │
│  ESP32 + I/O    │  │  RP2040 + MRAM  │  │ RC Forest Logic │
│                 │  │                 │  │                 │
│ • ESP32-S3      │  │ • RP2040        │  │ • LM393 Comps   │
│ • OLED Display  │  │ • 3x MRAM       │  │ • RC Networks   │
│ • SD Card       │  │ • SPI Bus       │  │ • Hall Sensors  │
│ • Power Input   │  │ • UART Link     │  │ • Tunnel Logic  │
└─────────────────┘  └─────────────────┘  └─────────────────┘
        │                       │                       │
        └───── Power Rails ─────┼───── Connected ──────┘
                                │
                        ┌─────────────────┐
                        │   CONTROL AREA  │
                        │                 │
                        │ • Magnets       │
                        │ • Test Probes   │
                        │ • Documentation │
                        └─────────────────┘
```

---

## 📋 Breadboard 1: ESP32-S3 + Display System

### Physical Layout (Top View)
```
Power Rails:  [+5V] ═══════════════════════════════════ [GND]
               │                                         │
Row 1:         │ ┌─ USB-C ─┐                           │
Row 2:         │ │ ESP32-S3│                           │
Row 3:         │ │ DevKit-C│                           │
Row 4:         │ │         │                           │
Row 5:         │ └─────────┘                           │
Row 6:         │                                       │
Row 7:         │                                       │
Row 8:         │ ┌─────────┐                           │
Row 9:         │ │  OLED   │                           │
Row 10:        │ │ Display │                           │
Row 11:        │ │ 128x64  │                           │
Row 12:        │ └─────────┘                           │
Row 13:        │                                       │
Row 14:        │                                       │
Row 15:        │ ┌─────────┐                           │
Row 16:        │ │SD Card  │                           │
Row 17:        │ │ Module  │                           │
Row 18:        │ └─────────┘                           │
Row 19:        │                                       │
Row 20:        │ [Wire Bundle to Breadboard 2]         │
               │                                       │
Power Rails:  [+5V] ═══════════════════════════════════ [GND]
```

### Component Details

**ESP32-S3 DevKit-C (Rows 2-5):**
- **Orientation**: USB-C connector pointing UP
- **Power**: VIN → +5V rail, GND → GND rail
- **Position**: Center of breadboard, pins fully inserted

**OLED Display (Rows 9-12):**
- **Orientation**: Screen facing UP
- **Connections**: 4-pin header (VCC, GND, SDA, SCL)
- **Position**: Left side, easy viewing angle

**SD Card Module (Rows 16-18):**
- **Orientation**: Card slot facing RIGHT
- **Connections**: 6-pin header (VCC, GND, MISO, MOSI, SCK, CS)
- **Position**: Right side, accessible for card insertion

### Wire Management
- **Use blue wires** for all ESP32 connections
- **Keep wires short** and organized
- **Route power wires** along breadboard edges
- **Bundle data wires** together with twist ties

---

## 📋 Breadboard 2: RP2040 + MRAM Array

### Physical Layout (Top View)
```
Power Rails:  [+3.3V] ═════════════════════════════════ [GND]
               │                                         │
Row 1:         │ [Wire Bundle from Breadboard 1]        │
Row 2:         │                                       │
Row 3:         │ ┌─ USB ─┐                             │
Row 4:         │ │RP2040 │                             │
Row 5:         │ │ Pico  │                             │
Row 6:         │ │       │                             │
Row 7:         │ └───────┘                             │
Row 8:         │                                       │
Row 9:         │                                       │
Row 10:        │ ┌─────┐ ┌─────┐ ┌─────┐               │
Row 11:        │ │MRAM │ │MRAM │ │MRAM │               │
Row 12:        │ │  A  │ │  B  │ │  C  │               │
Row 13:        │ └─────┘ └─────┘ └─────┘               │
Row 14:        │                                       │
Row 15:        │ [SPI Bus Wiring]                      │
Row 16:        │                                       │
Row 17:        │ [PWM Output Wiring]                   │
Row 18:        │                                       │
Row 19:        │ [ADC Input Wiring]                    │
Row 20:        │ [Wire Bundle to Breadboard 3]         │
               │                                       │
Power Rails:  [+3.3V] ═════════════════════════════════ [GND]
```

### Component Details

**Raspberry Pi Pico (Rows 4-7):**
- **Orientation**: Micro-USB connector pointing UP
- **Power**: VSYS (pin 39) → +5V, GND (pin 38) → GND
- **Position**: Center-left, easy access to all pins

**MRAM Array (Rows 11-13):**
- **MRAM A**: Left position (Symbolic Register A)
- **MRAM B**: Center position (Symbolic Register B)  
- **MRAM C**: Right position (Symbolic Register C)
- **Orientation**: Pin 1 (dot) toward TOP of breadboard
- **Adapters**: Use SOIC-8 to DIP if needed

### SPI Bus Wiring (Row 15)
```
Shared SPI Connections:
RP2040 GPIO2 (SCK)  ──┬── MRAM A SCK
                      ├── MRAM B SCK
                      └── MRAM C SCK

RP2040 GPIO3 (MOSI) ──┬── MRAM A MOSI
                      ├── MRAM B MOSI  
                      └── MRAM C MOSI

RP2040 GPIO4 (MISO) ──┬── MRAM A MISO
                      ├── MRAM B MISO
                      └── MRAM C MISO

Individual Chip Selects:
RP2040 GPIO5 ── MRAM A CS
RP2040 GPIO6 ── MRAM B CS
RP2040 GPIO7 ── MRAM C CS
```

### Wire Management
- **Use green wires** for all RP2040 connections
- **Keep SPI bus neat** with parallel wire routing
- **Label chip select wires** (A, B, C)
- **Use different colored wires** for PWM outputs

---

## 📋 Breadboard 3: RC Forest + Tunnel Logic

### Physical Layout (Top View)
```
Power Rails:  [+3.3V] ═════════════════════════════════ [GND]
               │                                         │
Row 1:         │ [Wire Bundle from Breadboard 2]        │
Row 2:         │                                       │
Row 3:         │ ┌─────────┐ ┌─────────┐               │
Row 4:         │ │ LM393#1 │ │ LM393#2 │               │
Row 5:         │ └─────────┘ └─────────┘               │
Row 6:         │                                       │
Row 7:         │ [RC Forest Node A] [RC Forest Node B] │
Row 8:         │ R=1kΩ C=1µF D=1N4148  R=2.2kΩ C=2.2µF│
Row 9:         │                                       │
Row 10:        │ [RC Forest Node C] [RC Forest Node D] │
Row 11:        │ R=4.7kΩ C=4.7µF      R=10kΩ C=10µF   │
Row 12:        │                                       │
Row 13:        │ ┌─────────┐ ┌─────────┐               │
Row 14:        │ │ Hall #1 │ │ Hall #2 │               │
Row 15:        │ │ A3144   │ │ A3144   │               │
Row 16:        │ └─────────┘ └─────────┘               │
Row 17:        │                                       │
Row 18:        │ [Magnet A] [Magnet B]                 │
Row 19:        │ (Adjustable Position)                 │
Row 20:        │                                       │
               │                                       │
Power Rails:  [+3.3V] ═════════════════════════════════ [GND]
```

### RC Forest Node Details

**Node A (Row 7-8) - Fast Decay:**
```
PWM Input ──┬── 1kΩ Resistor ──┬── 1µF Capacitor ──┬── Output
            │                  │                   │
            └── 1N4148 Diode ──┘                   │
                      │                            │
                      └── GND                      └── LM393 Pin 2
```

**Node B (Row 7-8) - Medium Decay:**
```
PWM Input ──┬── 2.2kΩ Resistor ──┬── 2.2µF Capacitor ──┬── Output
            │                    │                     │
            └── 1N4148 Diode ────┘                     │
                      │                                │
                      └── GND                          └── LM393 Pin 3
```

**Node C (Row 10-11) - Slow Decay:**
```
PWM Input ──┬── 4.7kΩ Resistor ──┬── 4.7µF Capacitor ──┬── Output
            │                    │                     │
            └── 1N4148 Diode ────┘                     │
                      │                                │
                      └── GND                          └── LM393 Pin 5
```

**Node D (Row 10-11) - Very Slow Decay:**
```
PWM Input ──┬── 10kΩ Resistor ──┬── 10µF Capacitor ──┬── Output
            │                   │                    │
            └── 1N4148 Diode ───┘                    │
                      │                              │
                      └── GND                        └── LM393 Pin 6
```

### Tunnel Logic Details

**Hall Sensor #1 (Rows 14-16):**
- **Position**: Left side of breadboard
- **Magnet A**: Positioned ~5mm away, adjustable
- **Output**: Connected to RP2040 GPIO14
- **Purpose**: Tunnel state detection A

**Hall Sensor #2 (Rows 14-16):**
- **Position**: Right side of breadboard  
- **Magnet B**: Positioned ~5mm away, adjustable
- **Output**: Connected to RP2040 GPIO15
- **Purpose**: Tunnel state detection B

### Wire Management
- **Use yellow wires** for RC forest connections
- **Use purple wires** for tunnel logic
- **Keep component leads short** for stability
- **Use breadboard jumpers** for neat connections

---

## 🔧 Physical Assembly Tips

### 1. Component Orientation Guide

**Integrated Circuits (ESP32, RP2040, LM393):**
- **Pin 1 indicator** (dot or notch) goes to TOP-LEFT
- **USB connectors** point UP for easy access
- **Ensure all pins** are fully inserted and straight

**Diodes (1N4148):**
- **Stripe (cathode)** toward GROUND connection
- **Body (anode)** toward SIGNAL connection
- **Keep leads short** to prevent oscillation

**Capacitors:**
- **Electrolytic**: Negative stripe toward GROUND
- **Ceramic**: No polarity, either direction OK
- **Keep close** to associated resistors

**MRAM Chips:**
- **Pin 1 (dot)** toward TOP of breadboard
- **Use adapters** if surface mount packages
- **Handle with anti-static** precautions

### 2. Wire Routing Best Practices

**Power Wires (Red/Black):**
- **Route along edges** of breadboards
- **Use thicker gauge** (22 AWG) for power
- **Keep power and ground** close together

**Signal Wires (Colored):**
- **Use different colors** for different functions
- **Keep wires short** to reduce noise
- **Avoid crossing** power wires when possible

**Bus Connections:**
- **Group related signals** (SPI bus, I²C bus)
- **Use parallel routing** for bus wires
- **Label wire bundles** with tape

### 3. Mechanical Stability

**Component Mounting:**
- **Press firmly** but don't force components
- **Check for loose connections** regularly
- **Use breadboard jumpers** for permanent connections

**Magnet Positioning:**
- **Use small stands** or tape for positioning
- **Mark optimal positions** with tape
- **Keep control magnets** easily accessible

**Cable Management:**
- **Use twist ties** for wire bundles
- **Leave slack** for adjustments
- **Route USB cables** to avoid interference

---

## 📏 Breadboard Specifications

### Recommended Breadboard Size
- **Type**: Large breadboards (830 tie points)
- **Dimensions**: 165mm x 55mm minimum
- **Power Rails**: Dual rails on both sides
- **Tie Points**: 630 main + 200 power rail

### Alternative Layouts

**Compact Version (2 Breadboards):**
- Combine ESP32 + RP2040 on one large breadboard
- Use second breadboard for RC forest + sensors
- Reduces wire length but increases complexity

**Expanded Version (4 Breadboards):**
- Separate each major subsystem
- Easier troubleshooting and modification
- Better organization for learning

**PCB Version:**
- Design custom PCB after breadboard testing
- Include mounting holes for sensors
- Add test points for debugging

---

## 🎯 Layout Verification Checklist

### Before Power-On
- [ ] All components oriented correctly
- [ ] All power connections verified
- [ ] No short circuits detected
- [ ] All signal connections complete
- [ ] Wire routing neat and secure

### Component Placement
- [ ] ESP32-S3 in correct position and orientation
- [ ] RP2040 in correct position and orientation
- [ ] All 3 MRAM chips properly seated
- [ ] LM393 comparators oriented correctly
- [ ] Hall sensors positioned with magnets

### Wire Connections
- [ ] Power rails connected between breadboards
- [ ] SPI bus wiring complete and correct
- [ ] I²C connections verified
- [ ] UART link between MCUs working
- [ ] PWM outputs connected to RC forest
- [ ] ADC inputs connected to sensors

### Mechanical Assembly
- [ ] All components firmly seated
- [ ] No loose wires or connections
- [ ] Magnets positioned correctly
- [ ] USB cables connected and secure
- [ ] Power supply adequate (5V, 2A minimum)

---

## 🔍 Testing Each Section

### Power System Test
```bash
# Use multimeter to verify:
Breadboard 1: +5V = 4.8-5.2V ✓
Breadboard 2: +5V = 4.8-5.2V ✓  
Breadboard 3: +5V = 4.8-5.2V ✓
All GND = 0V ✓
```

### Communication Test
```bash
# ESP32 ↔ RP2040 UART
ESP32> send "hello"
RP2040> received "hello" ✓

# ESP32 ↔ OLED I²C  
ESP32> oled_test
OLED> "Test Pattern" displayed ✓

# RP2040 ↔ MRAM SPI
RP2040> mram_test
MRAM A,B,C> Read/Write OK ✓
```

### Sensor Test
```bash
# RC Forest PWM → ADC
RP2040> pwm_test 50%
ADC0,1,2> Voltage levels changing ✓

# Hall Sensors
RP2040> hall_test
Move magnets> Digital outputs changing ✓
```

---

**Next Step**: See `assembly_guide.md` for detailed step-by-step build instructions!

**Need Help?** Check `troubleshooting.md` for common layout issues and solutions.
