# UPC v2.2 PRO - Troubleshooting Guide

## 🔧 Common Issues and Solutions

This guide helps you solve the most common problems when building and testing your UPC v2.2 PRO Symbolic Entropy Computer.

---

## ⚡ Power Issues

### Problem: No Power LEDs on MCUs
**Symptoms:**
- ESP32-S3 power LED off
- RP2040 power LED off
- OLED display blank

**Solutions:**
1. **Check power supply**: Verify 5V output with multimeter
2. **Check power rails**: Ensure +5V and GND connected across all breadboards
3. **Check MCU power pins**: 
   - ESP32: VIN → +5V, GND → GND
   - RP2040: VSYS (pin 39) → +5V, GND (pin 38) → GND
4. **Check for short circuits**: Use multimeter continuity test

### Problem: Voltage Levels Wrong
**Symptoms:**
- Multimeter shows incorrect voltages
- Components getting hot
- Erratic behavior

**Solutions:**
1. **Disconnect all loads** and measure power supply alone
2. **Check for reversed polarity** on power connections
3. **Verify power supply capacity**: Need minimum 2A at 5V
4. **Check breadboard power rail connections**

---

## 🔌 Communication Issues

### Problem: ESP32-S3 Won't Program
**Symptoms:**
- esptool.py fails to connect
- "Failed to connect" errors
- USB device not recognized

**Solutions:**
1. **Check USB-C cable**: Try different cable (data + power)
2. **Install drivers**: ESP32-S3 USB drivers for your OS
3. **Hold BOOT button**: Hold during programming, release after
4. **Check COM port**: Verify correct port in device manager
5. **Try different USB port**: Some ports have better power

### Problem: RP2040 Won't Program
**Symptoms:**
- RPI-RP2 drive doesn't appear
- .uf2 file copy fails
- Pico not recognized

**Solutions:**
1. **BOOTSEL procedure**: Hold BOOTSEL, connect USB, release BOOTSEL
2. **Check micro-USB cable**: Must support data transfer
3. **Try different computer**: Some USB controllers have issues
4. **Reset Pico**: Disconnect power, reconnect while holding BOOTSEL

### Problem: UART Communication Fails
**Symptoms:**
- ESP32 and RP2040 can't communicate
- No response to commands
- Garbled text output

**Solutions:**
1. **Check UART wiring**:
   - ESP32 GPIO1 (TX) → RP2040 GPIO0 (RX)
   - ESP32 GPIO2 (RX) → RP2040 GPIO1 (TX)
2. **Verify baud rate**: Both MCUs set to same rate (115200)
3. **Check ground connection**: Common ground between MCUs
4. **Test with loopback**: Connect TX to RX on same MCU

---

## 💾 MRAM Issues

### Problem: MRAM Not Detected
**Symptoms:**
- SPI test fails
- "No MRAM found" errors
- Read/write operations fail

**Solutions:**
1. **Check SOIC adapter**: Ensure MRAM chip properly seated
2. **Verify SPI wiring**:
   - All MRAM VCC → RP2040 3.3V
   - All MRAM GND → RP2040 GND
   - Shared MISO, MOSI, SCK connections
   - Individual CS lines to GPIO5, 6, 7
3. **Check chip orientation**: Pin 1 (dot) should be consistent
4. **Test one MRAM at a time**: Disconnect others to isolate

### Problem: MRAM Read/Write Errors
**Symptoms:**
- Data corruption
- Inconsistent reads
- Write operations fail

**Solutions:**
1. **Check power stability**: MRAM sensitive to voltage fluctuations
2. **Reduce SPI speed**: Try slower clock rate initially
3. **Add decoupling capacitors**: 100nF near each MRAM VCC pin
4. **Check for noise**: Keep SPI wires short and away from PWM

### Problem: Only Some MRAM Chips Work
**Symptoms:**
- MRAM A works, B and C fail
- Inconsistent chip detection
- Some addresses inaccessible

**Solutions:**
1. **Check individual CS lines**: Each MRAM needs unique chip select
2. **Verify CS timing**: Ensure proper setup/hold times
3. **Test CS signals with scope**: Should be clean digital transitions
4. **Swap MRAM positions**: Isolate chip vs. wiring issues

---

## 🌲 RC Forest Issues

### Problem: No PWM Output
**Symptoms:**
- RC forest nodes don't respond
- Constant voltage on PWM pins
- No entropy decay observed

**Solutions:**
1. **Check PWM configuration**: Verify RP2040 PWM setup in firmware
2. **Test PWM with oscilloscope**: Should see square wave output
3. **Verify GPIO pin assignments**:
   - GPIO8 → Forest Node A
   - GPIO9 → Forest Node B
   - GPIO10 → Forest Node C
   - GPIO11 → Forest Node D
4. **Check for conflicting pin usage**: Ensure pins not used elsewhere

### Problem: RC Time Constants Wrong
**Symptoms:**
- Decay too fast or too slow
- Comparator outputs don't change
- Entropy levels incorrect

**Solutions:**
1. **Verify component values**:
   - Node A: 1kΩ + 1µF = 1ms
   - Node B: 2.2kΩ + 2.2µF = 5ms
   - Node C: 4.7kΩ + 4.7µF = 22ms
   - Node D: 10kΩ + 10µF = 100ms
2. **Check component orientation**: Electrolytic caps have polarity
3. **Measure actual values**: Use multimeter to verify R and C
4. **Replace suspect components**: Some may be out of tolerance

### Problem: Comparator Issues
**Symptoms:**
- LM393 outputs stuck high or low
- No threshold detection
- Erratic switching behavior

**Solutions:**
1. **Check LM393 power**: Pin 8 → +5V, Pin 4 → GND
2. **Verify input connections**: Pins 2,3,5,6 to RC networks
3. **Check output connections**: Pins 1,7 to RP2040 GPIO12,13
4. **Add pull-up resistors**: 10kΩ from outputs to +5V
5. **Test with known voltages**: Apply fixed voltages to inputs

---

## 🧲 Tunnel Logic Issues

### Problem: Hall Sensors Not Working
**Symptoms:**
- No response to magnets
- Constant digital output
- ADC readings don't change

**Solutions:**
1. **Check hall sensor power**: VCC → +3.3V, GND → GND
2. **Verify magnet polarity**: Try flipping magnet orientation
3. **Adjust magnet distance**: Start close (~2mm) then move away
4. **Test with stronger magnets**: N52 neodymium recommended
5. **Check sensor type**: Ensure A3144 or compatible

### Problem: Magnetic Field Interference
**Symptoms:**
- Erratic sensor readings
- False triggers
- System instability

**Solutions:**
1. **Isolate magnetic sources**: Keep magnets away from other components
2. **Shield sensitive circuits**: Use metal enclosure if needed
3. **Separate hall sensors**: Increase distance between sensors
4. **Check for magnetic materials**: Remove steel tools from area
5. **Use magnetic shielding**: Mu-metal sheets around sensors

---

## 📺 Display Issues

### Problem: OLED Display Blank
**Symptoms:**
- No text or graphics on OLED
- Display backlight may be on
- I²C communication fails

**Solutions:**
1. **Check I²C wiring**:
   - OLED VCC → ESP32 3.3V
   - OLED GND → ESP32 GND
   - OLED SDA → ESP32 GPIO5
   - OLED SCL → ESP32 GPIO6
2. **Verify I²C address**: Usually 0x3C or 0x3D
3. **Check pull-up resistors**: May need 4.7kΩ on SDA/SCL
4. **Test with I²C scanner**: Verify device responds

### Problem: Display Corruption
**Symptoms:**
- Garbled text or graphics
- Partial display updates
- Random pixels

**Solutions:**
1. **Check power stability**: OLED sensitive to voltage drops
2. **Reduce I²C speed**: Try slower clock rate
3. **Add decoupling capacitor**: 100µF near OLED power pins
4. **Check for noise**: Keep I²C wires away from PWM signals
5. **Reset display**: Power cycle or software reset

---

## 💿 SD Card Issues

### Problem: SD Card Not Detected
**Symptoms:**
- "No SD card" errors
- File system mount fails
- SPI communication errors

**Solutions:**
1. **Check SD card format**: Must be FAT32
2. **Verify SPI wiring**:
   - SD VCC → ESP32 3.3V
   - SD GND → ESP32 GND
   - SD MISO → ESP32 GPIO7
   - SD MOSI → ESP32 GPIO8
   - SD SCK → ESP32 GPIO9
   - SD CS → ESP32 GPIO10
3. **Try different SD card**: Some cards incompatible
4. **Check card size**: Use 4-32GB cards for best compatibility

---

## 🔬 System-Level Issues

### Problem: System Hangs or Crashes
**Symptoms:**
- MCUs stop responding
- Watchdog resets
- Erratic behavior

**Solutions:**
1. **Check power quality**: Use oscilloscope to check for noise
2. **Add more decoupling**: 100µF + 100nF on each power rail
3. **Reduce system load**: Disable features one by one
4. **Check for infinite loops**: Review firmware logic
5. **Monitor temperatures**: Ensure adequate cooling

### Problem: Symbolic Operations Fail
**Symptoms:**
- REPL commands don't work
- Entropy calculations wrong
- State transitions incorrect

**Solutions:**
1. **Verify firmware versions**: Ensure latest compatible versions
2. **Check MRAM data integrity**: Run memory tests
3. **Calibrate RC forest**: Adjust time constants
4. **Reset symbolic state**: Clear all MRAM and restart
5. **Check mathematical precision**: Verify floating-point operations

### Problem: Performance Issues
**Symptoms:**
- Slow response times
- Low throughput
- Timeouts

**Solutions:**
1. **Optimize SPI speed**: Increase clock rate gradually
2. **Reduce debug output**: Disable verbose logging
3. **Check for blocking operations**: Use interrupts instead of polling
4. **Monitor CPU usage**: Ensure adequate processing time
5. **Optimize algorithms**: Review symbolic computation efficiency

---

## 🧪 Testing and Debugging Tools

### Essential Test Equipment
- **Multimeter**: Voltage, current, continuity testing
- **Oscilloscope**: Signal timing and quality analysis
- **Logic analyzer**: Digital signal debugging
- **Function generator**: Test signal injection

### Software Debug Tools
- **Serial monitor**: UART communication debugging
- **I²C scanner**: Device detection and addressing
- **SPI analyzer**: Bus communication verification
- **Memory dump**: MRAM content inspection

### Test Procedures
1. **Power-on test**: Verify all voltages before connecting components
2. **Communication test**: Test each interface individually
3. **Component test**: Verify each component separately
4. **Integration test**: Test complete system functionality
5. **Stress test**: Run extended operations to verify stability

---

## 📞 Getting Help

### Before Asking for Help
1. **Read this guide completely**
2. **Check all connections** with multimeter
3. **Test components individually**
4. **Document the problem** with photos and measurements
5. **Try the simplest solution first**

### What to Include in Help Requests
- **Clear problem description**
- **Photos of your build** (high resolution)
- **Multimeter readings** (voltages, continuity)
- **Error messages** (exact text)
- **Component list** (part numbers, sources)
- **What you've already tried**

### Community Resources
- **PreBinary GitHub Issues**: Technical support
- **Electronics Forums**: General hardware help
- **Discord/Slack**: Real-time community support
- **Video calls**: Screen sharing for complex issues

---

## 🎯 Prevention Tips

### Build Quality
- **Use quality components** from reputable suppliers
- **Double-check connections** before applying power
- **Keep workspace clean** and organized
- **Use proper tools** (good multimeter, quality wires)
- **Take photos** of working configurations

### Maintenance
- **Check connections regularly** (breadboards can loosen)
- **Keep firmware updated** to latest versions
- **Monitor component temperatures** during operation
- **Clean contacts** with isopropyl alcohol if needed
- **Document modifications** for future reference

### Safety
- **Never exceed voltage ratings**
- **Use ESD protection** when handling components
- **Keep magnets away** from storage devices
- **Disconnect power** before making changes
- **Have fire extinguisher** nearby when testing

---

**Remember**: Building the UPC v2.2 PRO is a learning experience. Don't get discouraged by initial problems - they're part of the process! Each issue you solve makes you a better hardware engineer.

**Need immediate help?** Check the PreBinary community forums or GitHub issues for real-time support from other builders.
