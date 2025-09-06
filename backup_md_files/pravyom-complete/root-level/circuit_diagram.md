# UPC v2.2 PRO - Complete Circuit Diagram

## 🔌 Beginner-Friendly Wiring Guide

This guide shows you exactly how to connect every component. **Follow the color-coded sections** and you'll have a working symbolic entropy computer!

### 🎯 Overview: What We're Building

```
Power Rail (5V) ──┬── ESP32-S3 ──┬── OLED Display
                  │               ├── SD Card Module
                  │               └── UART Debug
                  │
                  ├── RP2040 ────┬── MRAM Array (3x)
                  │              ├── RC Forest Network
                  │              ├── LM393 Comparators
                  │              └── Hall Sensors
                  │
                  └── Ground Rail ── Common Ground
```

---

## 📋 Section 1: Power Distribution (RED WIRES)

### Power Rails Setup
```
USB 5V Input ──┬── Breadboard Power Rail (+)
               └── Breadboard Power Rail (-)
```

**Connections:**
- **USB Power Supply** → **Breadboard Power Rails**
  - Red wire: +5V → Positive rail
  - Black wire: GND → Negative rail

**Power Distribution:**
- **ESP32-S3**: 5V + GND (via USB-C or VIN pin)
- **RP2040**: 5V + GND (via VSYS pin 39 + GND pin 38)
- **All Components**: Connect to power rails as needed

---

## 📋 Section 2: ESP32-S3 Connections (BLUE WIRES)

### ESP32-S3 DevKit-C Pinout
```
ESP32-S3 DevKit-C
┌─────────────────────┐
│ 3V3  │ GND  │ GPIO0│  ← Programming pins
│ GPIO1│ GPIO2│ GPIO3│
│ GPIO4│ GPIO5│ GPIO6│  ← I²C for OLED
│ GPIO7│ GPIO8│ GPIO9│  ← SPI for SD Card
│ ...  │ ...  │ ... │
└─────────────────────┘
```

### OLED Display (I²C)
- **OLED VCC** ← **ESP32 3.3V** (Pin 3V3)
- **OLED GND** ← **ESP32 GND** (Any GND pin)
- **OLED SDA** ← **ESP32 GPIO5** (I²C Data)
- **OLED SCL** ← **ESP32 GPIO6** (I²C Clock)

### SD Card Module (SPI)
- **SD VCC** ← **ESP32 3.3V**
- **SD GND** ← **ESP32 GND**
- **SD MISO** ← **ESP32 GPIO7**
- **SD MOSI** ← **ESP32 GPIO8**
- **SD SCK** ← **ESP32 GPIO9**
- **SD CS** ← **ESP32 GPIO10**

### UART Communication with RP2040
- **ESP32 GPIO1 (TX)** ← **RP2040 GPIO0 (RX)**
- **ESP32 GPIO2 (RX)** ← **RP2040 GPIO1 (TX)**

---

## 📋 Section 3: RP2040 Connections (GREEN WIRES)

### Raspberry Pi Pico Pinout
```
Raspberry Pi Pico (RP2040)
┌─────────────────────────┐
│GP0 │GP1 │GND│GP2 │GP3 │  ← UART + SPI start
│GP4 │GP5 │GP6 │GP7 │GP8 │  ← SPI for MRAM
│GP9 │GP10│GP11│GP12│GP13│  ← More SPI + PWM
│...│...│...│...│...│    │
│3V3│VSYS│GND│...│...│   │  ← Power pins
└─────────────────────────┘
```

### MRAM Array (3x MR25H256) - SPI Chain
**MRAM #1 (Symbolic Register A):**
- **MRAM VCC** ← **RP2040 3.3V** (Pin 36)
- **MRAM GND** ← **RP2040 GND** (Pin 38)
- **MRAM MISO** ← **RP2040 GPIO4** (SPI RX)
- **MRAM MOSI** ← **RP2040 GPIO3** (SPI TX)
- **MRAM SCK** ← **RP2040 GPIO2** (SPI Clock)
- **MRAM CS** ← **RP2040 GPIO5** (Chip Select A)

**MRAM #2 (Symbolic Register B):**
- **Power/Ground**: Same as MRAM #1
- **MISO/MOSI/SCK**: Same as MRAM #1 (shared SPI bus)
- **MRAM CS** ← **RP2040 GPIO6** (Chip Select B)

**MRAM #3 (Symbolic Register C):**
- **Power/Ground**: Same as MRAM #1
- **MISO/MOSI/SCK**: Same as MRAM #1 (shared SPI bus)
- **MRAM CS** ← **RP2040 GPIO7** (Chip Select C)

### PWM Outputs for RC Forest
- **PWM A** ← **RP2040 GPIO8** (Forest Node A)
- **PWM B** ← **RP2040 GPIO9** (Forest Node B)
- **PWM C** ← **RP2040 GPIO10** (Forest Node C)
- **PWM D** ← **RP2040 GPIO11** (Forest Node D)

### ADC Inputs for Sensing
- **ADC0** ← **RP2040 GPIO26** (Entropy Level A)
- **ADC1** ← **RP2040 GPIO27** (Entropy Level B)
- **ADC2** ← **RP2040 GPIO28** (Tunnel Field Strength)

---

## 📋 Section 4: RC Forest Network (YELLOW WIRES)

### Forest Node A (Entropy Decay Circuit)
```
RP2040 GPIO8 (PWM) ──┬── 1kΩ ──┬── 1µF ──┬── LM393 Pin 2 (+)
                     │         │         │
                     └── 1N4148 Diode ────┘
                               │
                               └── GND
```

### Forest Node B (Similar to A)
```
RP2040 GPIO9 (PWM) ──┬── 2.2kΩ ──┬── 2.2µF ──┬── LM393 Pin 3 (-)
                     │           │           │
                     └── 1N4148 Diode ──────┘
                               │
                               └── GND
```

### Forest Node C & D (Repeat pattern)
- **Node C**: 4.7kΩ + 4.7µF + Diode
- **Node D**: 10kΩ + 10µF + Diode

### LM393 Comparator Outputs
- **LM393 #1 Output** ← **RP2040 GPIO12** (Collapse Trigger A)
- **LM393 #2 Output** ← **RP2040 GPIO13** (Collapse Trigger B)

---

## 📋 Section 5: Tunnel Logic System (PURPLE WIRES)

### Hall Effect Sensors
**Hall Sensor #1:**
- **VCC** ← **RP2040 3.3V**
- **GND** ← **RP2040 GND**
- **OUT** ← **RP2040 GPIO14** (Tunnel State A)

**Hall Sensor #2:**
- **VCC** ← **RP2040 3.3V**
- **GND** ← **RP2040 GND**
- **OUT** ← **RP2040 GPIO15** (Tunnel State B)

### Magnet Placement
- **Magnet A**: Near Hall Sensor #1 (adjustable distance)
- **Magnet B**: Near Hall Sensor #2 (adjustable distance)
- **Control Magnets**: Hand-held for field manipulation

---

## 📋 Section 6: Complete Breadboard Layout

### Breadboard #1: Power + ESP32-S3
```
Power Rails:  [+5V] ═══════════════════ [GND]
               │                         │
Row 1:    ESP32-S3 DevKit-C (USB-C up)
Row 5:    OLED Display (4 pins)
Row 8:    SD Card Module (6 pins)
Row 12:   Jumper wires to Breadboard #2
```

### Breadboard #2: RP2040 + MRAM
```
Power Rails:  [+3.3V] ═══════════════════ [GND]
               │                           │
Row 1:    Raspberry Pi Pico (USB up)
Row 8:    MRAM #1 (SOIC adapter)
Row 12:   MRAM #2 (SOIC adapter)
Row 16:   MRAM #3 (SOIC adapter)
Row 20:   UART connection to ESP32
```

### Breadboard #3: RC Forest + Sensors
```
Power Rails:  [+3.3V] ═══════════════════ [GND]
               │                           │
Row 1:    LM393 Comparator #1
Row 5:    LM393 Comparator #2
Row 10:   RC Network (Resistors + Caps)
Row 15:   Hall Sensors + Magnets
Row 20:   Diode protection array
```

---

## 🔧 Assembly Tips for Beginners

### 1. **Start with Power**
- Connect power rails first
- Test with multimeter: 5V and 3.3V present
- **NEVER** connect power backwards!

### 2. **Add Components One by One**
- ESP32-S3 first (test OLED)
- RP2040 second (test UART communication)
- MRAM third (test SPI communication)
- RC Forest last (test PWM + ADC)

### 3. **Use Color-Coded Wires**
- **Red**: +5V power
- **Black**: Ground
- **Blue**: ESP32 connections
- **Green**: RP2040 connections
- **Yellow**: RC Forest
- **Purple**: Tunnel logic
- **White**: Data signals

### 4. **Double-Check Before Power On**
- All power connections correct
- No short circuits
- All grounds connected
- Components oriented correctly

---

## 🧪 Testing Each Section

### Power Test
```bash
# Check voltages with multimeter
+5V Rail: 4.8V - 5.2V ✓
+3.3V Rail: 3.1V - 3.4V ✓
All GND connected ✓
```

### ESP32 Test
```bash
# Flash test firmware
esptool.py --chip esp32s3 write_flash 0x0 test_esp32.bin
# Should see OLED display "ESP32 OK"
```

### RP2040 Test
```bash
# Flash test firmware
picotool load test_rp2040.uf2
# Should see UART output "RP2040 OK"
```

### MRAM Test
```bash
# Run SPI test
# Should read/write to all 3 MRAM chips
MRAM A: OK ✓
MRAM B: OK ✓  
MRAM C: OK ✓
```

### Complete System Test
```bash
# Load full firmware
# Should see symbolic REPL prompt
UPC v2.2 PRO > _
```

---

## 🚨 Safety Notes

- **Never exceed 5V** on any component
- **Always connect ground first** when wiring
- **Use ESD protection** when handling MRAM chips
- **Keep magnets away** from storage devices
- **Double-check polarity** before applying power

---

**Next Step**: See `assembly_guide.md` for detailed step-by-step instructions!
