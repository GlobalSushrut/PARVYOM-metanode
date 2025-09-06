# UPC v2.2 PRO - Step-by-Step Assembly Guide

## 🎯 Complete Build Instructions for Beginners

**Estimated Build Time**: 2-4 hours  
**Difficulty Level**: Beginner (No soldering required)  
**Required Skills**: Basic electronics knowledge helpful but not required

---

## 📦 Pre-Assembly Checklist

### ✅ Components Verification
Print this checklist and check off each item as you unpack:

**Core Processing Units:**
- [ ] 3x Everspin MR25H256 MRAM chips
- [ ] 3-5x SOIC-8 to DIP adapters (if MRAM is surface mount)
- [ ] 1x Raspberry Pi Pico (RP2040)
- [ ] 1x ESP32-S3 DevKit-C

**Logic Components:**
- [ ] 2x LM393 Dual Comparator (DIP package)
- [ ] 8-10x 1N4148 Diodes
- [ ] 20x Mixed Resistors (100Ω, 1kΩ, 2.2kΩ, 4.7kΩ, 10kΩ)
- [ ] 20x Mixed Capacitors (10nF, 100nF, 1µF, 2.2µF, 4.7µF, 10µF)

**Sensors & Display:**
- [ ] 2x Hall Effect Sensors (A3144)
- [ ] 4x Neodymium Magnets (N52 discs)
- [ ] 1x OLED Display (SSD1306 128x64, I²C)
- [ ] 1x SD Card Module (SPI)
- [ ] 1x microSD Card (4-32GB, formatted FAT32)

**Assembly Materials:**
- [ ] 3x Large Breadboards (830 tie points recommended)
- [ ] 1x Jumper wire kit (male-male, male-female)
- [ ] 1x Header pin strips (2.54mm pitch)
- [ ] 1x USB-C cable (ESP32)
- [ ] 1x Micro-USB cable (RP2040)
- [ ] 1x 5V USB power supply (2A minimum)

**Tools Required:**
- [ ] Multimeter (for testing)
- [ ] Small screwdriver set
- [ ] Wire strippers (optional)
- [ ] Anti-static wrist strap (recommended)

---

## 🔧 Step 1: Workspace Setup (15 minutes)

### 1.1 Prepare Your Workspace
```
┌─────────────────────────────────────┐
│  Clean, Well-Lit Workspace         │
│                                     │
│  [Breadboard 1] [Breadboard 2]     │
│                                     │
│  [Breadboard 3] [Component Tray]    │
│                                     │
│  [Tools]        [Documentation]     │
└─────────────────────────────────────┘
```

**Setup Tasks:**
1. **Clear a large, flat surface** (minimum 2 feet x 3 feet)
2. **Arrange good lighting** (desk lamp recommended)
3. **Organize components** in small containers or anti-static bags
4. **Have documentation ready** (this guide + circuit diagram)
5. **Wear anti-static protection** if available

### 1.2 Label Your Breadboards
Use masking tape to label:
- **Breadboard 1**: "ESP32 + Display"
- **Breadboard 2**: "RP2040 + MRAM"
- **Breadboard 3**: "RC Forest + Sensors"

---

## 🔧 Step 2: Power Distribution Setup (20 minutes)

### 2.1 Set Up Power Rails

**On ALL Three Breadboards:**
1. **Connect power rails** using red and black jumper wires
2. **Link positive rails** across all breadboards (red wires)
3. **Link negative rails** across all breadboards (black wires)

```
Breadboard Power Rail Connection:
[BB1] ═══ Red Wire ═══ [BB2] ═══ Red Wire ═══ [BB3]  (+5V)
[BB1] ═══ Black Wire ══ [BB2] ═══ Black Wire ══ [BB3]  (GND)
```

### 2.2 Test Power Distribution
1. **Connect USB power supply** to Breadboard 1 power rails
2. **Use multimeter** to verify:
   - **+5V rail**: 4.8V - 5.2V ✓
   - **GND rail**: 0V ✓
   - **Continuity**: All breadboards connected ✓

**⚠️ CRITICAL**: Do not proceed until power is verified correct!

---

## 🔧 Step 3: ESP32-S3 Setup (30 minutes)

### 3.1 Install ESP32-S3 on Breadboard 1

**Physical Placement:**
1. **Orient ESP32-S3** with USB-C connector facing up
2. **Insert into breadboard** starting at row 5
3. **Ensure pins are fully inserted** and straight
4. **Connect power**:
   - **VIN pin** → **+5V rail** (red wire)
   - **GND pin** → **GND rail** (black wire)

### 3.2 Connect OLED Display (I²C)

**OLED Wiring** (use blue wires for ESP32 connections):
```
OLED Display    ESP32-S3 Pin    Wire Color
VCC         →   3.3V           Red
GND         →   GND            Black  
SDA         →   GPIO5          Blue
SCL         →   GPIO6          Blue
```

**Physical Steps:**
1. **Place OLED** in rows 10-13 of Breadboard 1
2. **Connect VCC** to ESP32 3.3V output pin
3. **Connect GND** to ESP32 GND pin
4. **Connect SDA** to ESP32 GPIO5
5. **Connect SCL** to ESP32 GPIO6

### 3.3 Connect SD Card Module (SPI)

**SD Card Wiring**:
```
SD Module       ESP32-S3 Pin    Wire Color
VCC         →   3.3V           Red
GND         →   GND            Black
MISO        →   GPIO7          Blue
MOSI        →   GPIO8          Blue
SCK         →   GPIO9          Blue
CS          →   GPIO10         Blue
```

### 3.4 Test ESP32-S3 Setup
1. **Connect USB-C cable** to ESP32-S3
2. **Power on** the system
3. **Check for**:
   - ESP32 power LED on ✓
   - OLED display initializes ✓
   - No smoke or heat ✓

---

## 🔧 Step 4: RP2040 Setup (30 minutes)

### 4.1 Install RP2040 on Breadboard 2

**Physical Placement:**
1. **Orient RP2040** with micro-USB connector facing up
2. **Insert into breadboard** starting at row 5
3. **Connect power**:
   - **VSYS pin (39)** → **+5V rail** (red wire)
   - **GND pin (38)** → **GND rail** (black wire)

### 4.2 Set Up UART Communication

**UART Wiring** between ESP32 and RP2040:
```
ESP32-S3        RP2040          Wire Color
GPIO1 (TX)  →   GPIO0 (RX)     Green
GPIO2 (RX)  →   GPIO1 (TX)     Green
```

**Physical Steps:**
1. **Run green wires** between Breadboard 1 and 2
2. **Connect ESP32 GPIO1** to **RP2040 GPIO0**
3. **Connect ESP32 GPIO2** to **RP2040 GPIO1**
4. **Ensure solid connections** in breadboard tie points

### 4.3 Test RP2040 Setup
1. **Connect micro-USB cable** to RP2040
2. **Power on** the system
3. **Check for**:
   - RP2040 power LED on ✓
   - UART communication working ✓
   - Both MCUs powered simultaneously ✓

---

## 🔧 Step 5: MRAM Array Setup (45 minutes)

### 5.1 Prepare MRAM Chips

**If using SOIC-8 packages:**
1. **Insert MRAM chips** into SOIC-8 to DIP adapters
2. **Ensure proper orientation** (pin 1 marked with dot)
3. **Check for solid connections** between chip and adapter

### 5.2 Install MRAM Array

**Physical Placement on Breadboard 2:**
- **MRAM #1**: Rows 10-12 (Symbolic Register A)
- **MRAM #2**: Rows 15-17 (Symbolic Register B)  
- **MRAM #3**: Rows 20-22 (Symbolic Register C)

### 5.3 Wire MRAM SPI Bus

**Shared SPI Connections** (green wires):
```
All MRAM Chips  RP2040 Pin      Wire Color
VCC         →   3.3V (Pin 36)   Red
GND         →   GND (Pin 38)    Black
MISO        →   GPIO4           Green
MOSI        →   GPIO3           Green
SCK         →   GPIO2           Green
```

**Individual Chip Select Lines**:
```
MRAM Chip       RP2040 Pin      Wire Color
MRAM #1 CS  →   GPIO5           Green
MRAM #2 CS  →   GPIO6           Green
MRAM #3 CS  →   GPIO7           Green
```

### 5.4 Test MRAM Array
1. **Power on** the system
2. **Flash test firmware** to RP2040
3. **Verify SPI communication**:
   - MRAM #1: Read/Write OK ✓
   - MRAM #2: Read/Write OK ✓
   - MRAM #3: Read/Write OK ✓

---

## 🔧 Step 6: RC Forest Network (60 minutes)

### 6.1 Build Forest Node A

**Components for Node A:**
- 1x 1kΩ resistor
- 1x 1µF capacitor
- 1x 1N4148 diode

**Circuit Assembly:**
```
RP2040 GPIO8 ──┬── 1kΩ ──┬── 1µF ──┬── To LM393 Pin 2
               │         │         │
               └── 1N4148 Diode ────┘
                         │
                         └── GND
```

**Physical Steps:**
1. **Place components** in rows 5-8 of Breadboard 3
2. **Connect PWM input** from RP2040 GPIO8 (green wire)
3. **Build RC network** with resistor and capacitor
4. **Add diode protection** (stripe toward ground)
5. **Connect output** to LM393 comparator input

### 6.2 Build Forest Nodes B, C, D

**Node B** (rows 10-13):
- 2.2kΩ resistor + 2.2µF capacitor + 1N4148 diode
- PWM input: RP2040 GPIO9

**Node C** (rows 15-18):
- 4.7kΩ resistor + 4.7µF capacitor + 1N4148 diode  
- PWM input: RP2040 GPIO10

**Node D** (rows 20-23):
- 10kΩ resistor + 10µF capacitor + 1N4148 diode
- PWM input: RP2040 GPIO11

### 6.3 Install LM393 Comparators

**LM393 #1** (rows 25-27):
```
Pin 1: Output A     → RP2040 GPIO12
Pin 2: Input A+     → Forest Node A output
Pin 3: Input A-     → Forest Node B output  
Pin 4: GND          → Ground rail
Pin 5: Input B-     → Forest Node C output
Pin 6: Input B+     → Forest Node D output
Pin 7: Output B     → RP2040 GPIO13
Pin 8: VCC          → +5V rail
```

### 6.4 Test RC Forest
1. **Power on** the system
2. **Run PWM test** on all 4 channels
3. **Verify comparator outputs** change with PWM
4. **Check timing constants**:
   - Node A: ~1ms time constant ✓
   - Node B: ~5ms time constant ✓
   - Node C: ~20ms time constant ✓
   - Node D: ~100ms time constant ✓

---

## 🔧 Step 7: Tunnel Logic System (30 minutes)

### 7.1 Install Hall Effect Sensors

**Hall Sensor #1** (rows 30-32):
```
Pin 1: VCC      → +3.3V rail
Pin 2: GND      → Ground rail
Pin 3: OUT      → RP2040 GPIO14
```

**Hall Sensor #2** (rows 35-37):
```
Pin 1: VCC      → +3.3V rail  
Pin 2: GND      → Ground rail
Pin 3: OUT      → RP2040 GPIO15
```

### 7.2 Set Up Magnetic Field Control

**Magnet Placement:**
1. **Mount magnets** on small stands or tape
2. **Position Magnet A** ~5mm from Hall Sensor #1
3. **Position Magnet B** ~5mm from Hall Sensor #2
4. **Keep control magnets** handheld for field manipulation

### 7.3 Wire ADC Sensing

**ADC Connections**:
```
Sensor Input    RP2040 Pin      Purpose
Entropy A   →   GPIO26 (ADC0)   Forest Node A voltage
Entropy B   →   GPIO27 (ADC1)   Forest Node B voltage  
Tunnel      →   GPIO28 (ADC2)   Combined hall sensor output
```

### 7.4 Test Tunnel Logic
1. **Power on** the system
2. **Move magnets** near hall sensors
3. **Verify digital outputs** change with magnetic field
4. **Check ADC readings** reflect magnetic field strength

---

## 🔧 Step 8: Final Integration & Testing (45 minutes)

### 8.1 Complete System Wiring Check

**Use this checklist** to verify all connections:

**Power System:**
- [ ] All +5V rails connected
- [ ] All GND rails connected  
- [ ] No short circuits
- [ ] Voltage levels correct

**ESP32-S3 System:**
- [ ] OLED display connected (I²C)
- [ ] SD card module connected (SPI)
- [ ] UART to RP2040 connected
- [ ] Power connections secure

**RP2040 System:**
- [ ] All 3 MRAM chips connected (SPI)
- [ ] All 4 PWM outputs connected
- [ ] All 3 ADC inputs connected
- [ ] UART to ESP32 connected
- [ ] Comparator inputs/outputs connected

**RC Forest:**
- [ ] All 4 forest nodes built correctly
- [ ] All diodes oriented correctly
- [ ] LM393 comparators wired correctly
- [ ] PWM and ADC connections verified

**Tunnel Logic:**
- [ ] Hall sensors powered and connected
- [ ] Magnets positioned correctly
- [ ] ADC sensing connected

### 8.2 Flash Complete Firmware

**ESP32-S3 Firmware:**
```bash
# Connect ESP32-S3 via USB-C
esptool.py --chip esp32s3 write_flash 0x0 upc_esp32_v2.2.bin
```

**RP2040 Firmware:**
```bash
# Connect RP2040 via micro-USB  
# Hold BOOTSEL button, connect USB, release button
# Copy upc_rp2040_v2.2.uf2 to RPI-RP2 drive
```

### 8.3 System Startup Test

**Power On Sequence:**
1. **Connect both USB cables** (ESP32 + RP2040)
2. **Connect main power supply** to breadboard
3. **Watch for startup sequence**:
   - ESP32 boots, OLED shows "UPC v2.2 PRO"
   - RP2040 boots, initializes MRAM array
   - UART communication established
   - RC forest calibrates
   - Hall sensors initialize

**Expected Output:**
```
UPC v2.2 PRO - Symbolic Entropy Computer
========================================
MRAM Array: 3 chips detected ✓
RC Forest: 4 nodes calibrated ✓  
Tunnel Logic: 2 sensors active ✓
UART Link: ESP32 ↔ RP2040 ✓

Ready for symbolic computation!
UPC> _
```

### 8.4 Run Basic Tests

**Test 1: MRAM Symbolic Storage**
```
UPC> store A "hello_world" entropy=0.75
Stored symbolic morphon in MRAM A ✓

UPC> recall A
Retrieved: "hello_world" entropy=0.75 ✓
```

**Test 2: RC Forest Collapse**
```
UPC> forest collapse threshold=0.5
Forest Node A: Collapsed ✓
Forest Node B: Superposition
Forest Node C: Entangled  
Forest Node D: Void
```

**Test 3: Tunnel Logic**
```
UPC> tunnel scan
Hall Sensor A: Field detected (0.8T)
Hall Sensor B: No field (0.0T)
Tunnel state: Asymmetric ✓
```

---

## 🎉 Congratulations!

You have successfully built the **UPC v2.2 PRO Symbolic Entropy Computer**!

### 🚀 What You've Accomplished

- ✅ Built the world's first DIY symbolic entropy computer
- ✅ Implemented MRAM-based symbolic logic storage
- ✅ Created RC forest entropy decay networks
- ✅ Integrated magnetic tunnel logic control
- ✅ Established real-time symbolic REPL environment

### 🔬 Next Steps

1. **Explore symbolic programming** with the examples in `software/`
2. **Experiment with entropy manipulation** using the RC forest
3. **Try magnetic field programming** with the tunnel logic
4. **Build more complex symbolic applications**
5. **Share your results** with the PreBinary community!

### 📚 Advanced Modifications

- **Add more MRAM chips** for larger symbolic memory
- **Build custom PCB** for permanent installation
- **Add wireless modules** for remote symbolic control
- **Integrate sensors** for environmental symbolic input
- **Create symbolic neural networks** for AI applications

---

## 🆘 Troubleshooting

See `troubleshooting.md` for solutions to common issues.

**Need Help?**
- Check connections with multimeter
- Verify firmware versions
- Test components individually
- Review circuit diagram carefully

**Still Stuck?**
- Post photos of your build
- Share error messages
- Ask the PreBinary community

---

**Welcome to the future of symbolic computation!** 🚀🧠✨
