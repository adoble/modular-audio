EESchema Schematic File Version 4
LIBS:i2s-multiplexer-cache
EELAYER 30 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title "I2S Multiplexer - Modular Amp"
Date ""
Rev ""
Comp "Doble Audio"
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L 74xx:74HC4051 U4
U 1 1 61D09891
P 4250 4850
F 0 "U4" H 4550 4300 50  0000 C CNN
F 1 "74HC4051" H 4000 4300 50  0000 C CNN
F 2 "Package_SO:SOIC-16_3.9x9.9mm_P1.27mm" H 4250 4450 50  0001 C CNN
F 3 "http://www.ti.com/lit/ds/symlink/cd74hc4051.pdf" H 4250 4450 50  0001 C CNN
	1    4250 4850
	-1   0    0    1   
$EndComp
$Comp
L 74xx:74HC4051 U1
U 1 1 61D0A4DB
P 4200 6500
F 0 "U1" H 4500 5950 50  0000 C CNN
F 1 "74HC4051" H 3950 5950 50  0000 C CNN
F 2 "Package_SO:SOIC-16_3.9x9.9mm_P1.27mm" H 4200 6100 50  0001 C CNN
F 3 "http://www.ti.com/lit/ds/symlink/cd74hc4051.pdf" H 4200 6100 50  0001 C CNN
	1    4200 6500
	-1   0    0    1   
$EndComp
$Comp
L 74xx:74LS126 U5
U 1 1 61D0D21C
P 5550 1900
F 0 "U5" H 5550 2217 50  0000 C CNN
F 1 "74HC126" H 5550 2126 50  0000 C CNN
F 2 "Package_SO:SOIC-14_3.9x8.7mm_P1.27mm" H 5550 1900 50  0001 C CNN
F 3 "http://www.ti.com/lit/gpn/sn74LS126" H 5550 1900 50  0001 C CNN
	1    5550 1900
	1    0    0    -1  
$EndComp
$Comp
L 74xx:74LS126 U5
U 3 1 61D0E9B1
P 5550 5150
F 0 "U5" H 5550 5467 50  0000 C CNN
F 1 "74HC126" H 5550 5376 50  0000 C CNN
F 2 "Package_SO:SOIC-14_3.9x8.7mm_P1.27mm" H 5550 5150 50  0001 C CNN
F 3 "http://www.ti.com/lit/gpn/sn74LS126" H 5550 5150 50  0001 C CNN
	3    5550 5150
	1    0    0    -1  
$EndComp
$Comp
L 74xx:74LS126 U5
U 4 1 61D0F241
P 5550 6800
F 0 "U5" H 5550 7117 50  0000 C CNN
F 1 "74HC126" H 5550 7026 50  0000 C CNN
F 2 "Package_SO:SOIC-14_3.9x8.7mm_P1.27mm" H 5550 6800 50  0001 C CNN
F 3 "http://www.ti.com/lit/gpn/sn74LS126" H 5550 6800 50  0001 C CNN
	4    5550 6800
	1    0    0    -1  
$EndComp
Wire Wire Line
	4550 5150 5250 5150
$Comp
L power:+3.3V #PWR06
U 1 1 61D14209
P 4250 5350
F 0 "#PWR06" H 4250 5200 50  0001 C CNN
F 1 "+3.3V" H 4265 5523 50  0000 C CNN
F 2 "" H 4250 5350 50  0001 C CNN
F 3 "" H 4250 5350 50  0001 C CNN
	1    4250 5350
	-1   0    0    1   
$EndComp
$Comp
L power:+3.3V #PWR03
U 1 1 61D144E4
P 4200 7000
F 0 "#PWR03" H 4200 6850 50  0001 C CNN
F 1 "+3.3V" H 4215 7173 50  0000 C CNN
F 2 "" H 4200 7000 50  0001 C CNN
F 3 "" H 4200 7000 50  0001 C CNN
	1    4200 7000
	-1   0    0    1   
$EndComp
$Comp
L power:+3.3V #PWR014
U 1 1 61D14F62
P 6300 1250
F 0 "#PWR014" H 6300 1100 50  0001 C CNN
F 1 "+3.3V" H 6315 1423 50  0000 C CNN
F 2 "" H 6300 1250 50  0001 C CNN
F 3 "" H 6300 1250 50  0001 C CNN
	1    6300 1250
	0    -1   -1   0   
$EndComp
$Comp
L power:GND #PWR010
U 1 1 61D18967
P 4150 4250
F 0 "#PWR010" H 4150 4000 50  0001 C CNN
F 1 "GND" H 4200 4100 50  0000 C CNN
F 2 "" H 4150 4250 50  0001 C CNN
F 3 "" H 4150 4250 50  0001 C CNN
	1    4150 4250
	-1   0    0    1   
$EndComp
$Comp
L power:GND #PWR07
U 1 1 61D1972C
P 4200 5900
F 0 "#PWR07" H 4200 5650 50  0001 C CNN
F 1 "GND" H 4150 5750 50  0000 C CNN
F 2 "" H 4200 5900 50  0001 C CNN
F 3 "" H 4200 5900 50  0001 C CNN
	1    4200 5900
	-1   0    0    1   
$EndComp
Text Label 5550 7200 0    50   ~ 0
MCLK_EN
$Comp
L power:GND #PWR015
U 1 1 61D1D171
P 7300 1250
F 0 "#PWR015" H 7300 1000 50  0001 C CNN
F 1 "GND" H 7305 1077 50  0000 C CNN
F 2 "" H 7300 1250 50  0001 C CNN
F 3 "" H 7300 1250 50  0001 C CNN
	1    7300 1250
	0    -1   -1   0   
$EndComp
Text Label 6250 1900 0    50   ~ 0
LRCLK
Text Label 6200 3550 0    50   ~ 0
BCLK
Text Label 6250 5150 0    50   ~ 0
DATA
Text Label 6200 6800 0    50   ~ 0
MCLK
$Comp
L Connector_Generic:Conn_01x04 J2
U 1 1 61D23922
P 1050 4200
F 0 "J2" H 968 3775 50  0000 C CNN
F 1 "Conn_01x04" H 968 3866 50  0000 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x04_P2.54mm_Vertical" H 1050 4200 50  0001 C CNN
F 3 "~" H 1050 4200 50  0001 C CNN
	1    1050 4200
	-1   0    0    1   
$EndComp
Wire Wire Line
	1250 4000 1600 4000
Wire Wire Line
	1250 4100 1600 4100
Wire Wire Line
	1250 4200 1600 4200
Wire Wire Line
	1250 4300 1600 4300
Text Label 1250 4000 0    50   ~ 0
LRCLK_0
Text Label 1250 4100 0    50   ~ 0
BCLK_0
Text Label 1250 4200 0    50   ~ 0
DATA_0
Text Label 1250 4300 0    50   ~ 0
MCLK_0
$Comp
L Connector_Generic:Conn_01x04 J3
U 1 1 61D28DA5
P 1050 4950
F 0 "J3" H 968 4525 50  0000 C CNN
F 1 "Conn_01x04" H 968 4616 50  0000 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x04_P2.54mm_Vertical" H 1050 4950 50  0001 C CNN
F 3 "~" H 1050 4950 50  0001 C CNN
	1    1050 4950
	-1   0    0    1   
$EndComp
Wire Wire Line
	1250 4750 1600 4750
Wire Wire Line
	1250 4850 1600 4850
Wire Wire Line
	1250 4950 1600 4950
Wire Wire Line
	1250 5050 1600 5050
Text Label 1250 4750 0    50   ~ 0
LRCLK_1
Text Label 1250 4850 0    50   ~ 0
BCLK_1
Text Label 1250 4950 0    50   ~ 0
DATA_1
Text Label 1250 5050 0    50   ~ 0
MCLK_1
$Comp
L Connector_Generic:Conn_01x04 J4
U 1 1 61D2ACD6
P 1050 5750
F 0 "J4" H 968 5325 50  0000 C CNN
F 1 "Conn_01x04" H 968 5416 50  0000 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x04_P2.54mm_Vertical" H 1050 5750 50  0001 C CNN
F 3 "~" H 1050 5750 50  0001 C CNN
	1    1050 5750
	-1   0    0    1   
$EndComp
Wire Wire Line
	1250 5550 1600 5550
Wire Wire Line
	1250 5650 1600 5650
Wire Wire Line
	1250 5850 1600 5850
Text Label 1250 5550 0    50   ~ 0
LRCLK_2
Text Label 1250 5650 0    50   ~ 0
BCLK_2
Text Label 1250 5750 0    50   ~ 0
DATA_2
Text Label 1250 5850 0    50   ~ 0
MCLK_2
$Comp
L Connector_Generic:Conn_01x04 J5
U 1 1 61D2BB7E
P 1050 6500
F 0 "J5" H 968 6075 50  0000 C CNN
F 1 "Conn_01x04" H 968 6166 50  0000 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x04_P2.54mm_Vertical" H 1050 6500 50  0001 C CNN
F 3 "~" H 1050 6500 50  0001 C CNN
	1    1050 6500
	-1   0    0    1   
$EndComp
Wire Wire Line
	1250 6300 1600 6300
Wire Wire Line
	1250 6400 1600 6400
Wire Wire Line
	1250 6500 1600 6500
Wire Wire Line
	1250 6600 1600 6600
Text Label 1250 6300 0    50   ~ 0
LRCLK_3
Text Label 1250 6400 0    50   ~ 0
BCLK_3
Text Label 1250 6500 0    50   ~ 0
DATA_3
Text Label 1250 6600 0    50   ~ 0
MCLK_3
Text Label 3400 1900 0    50   ~ 0
LRCLK_0
Text Label 3400 1800 0    50   ~ 0
LRCLK_1
Text Label 3400 1700 0    50   ~ 0
LRCLK_2
Text Label 3400 1600 0    50   ~ 0
LRCLK_3
Text Label 3400 3550 0    50   ~ 0
BCLK_0
Text Label 3400 3450 0    50   ~ 0
BCLK_1
Text Label 3400 3350 0    50   ~ 0
BCLK_2
Text Label 3400 3250 0    50   ~ 0
BCLK_3
Wire Wire Line
	3850 5150 3300 5150
Wire Wire Line
	3850 5050 3300 5050
Wire Wire Line
	3850 4950 3300 4950
Wire Wire Line
	3850 4850 3300 4850
Text Label 3400 5150 0    50   ~ 0
DATA_0
Text Label 3400 5050 0    50   ~ 0
DATA_1
Text Label 3400 4950 0    50   ~ 0
DATA_2
Text Label 3400 4850 0    50   ~ 0
DATA_3
Wire Wire Line
	3800 6800 3300 6800
Wire Wire Line
	3800 6700 3300 6700
Wire Wire Line
	3800 6600 3300 6600
Wire Wire Line
	3800 6500 3300 6500
Text Label 3400 6800 0    50   ~ 0
MCLK_0
Text Label 3400 6700 0    50   ~ 0
MCLK_1
Text Label 3400 6600 0    50   ~ 0
MCLK_2
Text Label 3400 6500 0    50   ~ 0
MCLK_3
Wire Wire Line
	4500 6800 5250 6800
Entry Wire Line
	4800 1500 4900 1600
Entry Wire Line
	4800 1600 4900 1700
Entry Wire Line
	4800 1700 4900 1800
Entry Wire Line
	4800 3150 4900 3250
Entry Wire Line
	4800 3250 4900 3350
Entry Wire Line
	4800 3350 4900 3450
Entry Wire Line
	4800 4550 4900 4650
Entry Wire Line
	4800 4750 4900 4850
Entry Wire Line
	4800 4850 4900 4950
Entry Wire Line
	4800 4950 4900 5050
Entry Wire Line
	4800 2950 4900 3050
Entry Wire Line
	4800 6200 4900 6300
Entry Wire Line
	4800 6400 4900 6500
Entry Wire Line
	4800 6500 4900 6600
Entry Wire Line
	4800 6600 4900 6700
Entry Wire Line
	4800 1300 4900 1400
Text Label 4600 1300 0    50   ~ 0
~SRC_EN
Wire Wire Line
	4550 4550 4800 4550
Text Label 4600 2950 0    50   ~ 0
~SRC_EN
Text Label 4600 4550 0    50   ~ 0
~SRC_EN
Text Label 4550 6200 0    50   ~ 0
~SRC_EN
Wire Wire Line
	4500 6200 4800 6200
Text Label 4650 1500 0    50   ~ 0
S2
Text Label 4650 1600 0    50   ~ 0
S1
Text Label 4650 1700 0    50   ~ 0
S0
Text Label 4650 3150 0    50   ~ 0
S2
Text Label 4650 3250 0    50   ~ 0
S1
Text Label 4650 3350 0    50   ~ 0
S0
Wire Wire Line
	4550 4950 4800 4950
Text Label 4650 4750 0    50   ~ 0
S2
Text Label 4650 4850 0    50   ~ 0
S1
Text Label 4650 4950 0    50   ~ 0
S0
Wire Wire Line
	4550 4850 4800 4850
Wire Wire Line
	4550 4750 4800 4750
Text Label 4700 6400 0    50   ~ 0
S2
Text Label 4700 6500 0    50   ~ 0
S1
Text Label 4700 6600 0    50   ~ 0
S0
Wire Wire Line
	4500 6400 4800 6400
Wire Wire Line
	4500 6500 4800 6500
Wire Wire Line
	4500 6600 4800 6600
$Comp
L Device:R_Small R1
U 1 1 61D7F458
P 5550 2250
F 0 "R1" H 5609 2296 50  0000 L CNN
F 1 "10K" H 5609 2205 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 5550 2250 50  0001 C CNN
F 3 "~" H 5550 2250 50  0001 C CNN
	1    5550 2250
	1    0    0    -1  
$EndComp
Wire Wire Line
	5550 2350 5900 2350
$Comp
L power:+3.3V #PWR011
U 1 1 61D8252B
P 5900 2350
F 0 "#PWR011" H 5900 2200 50  0001 C CNN
F 1 "+3.3V" H 5915 2523 50  0000 C CNN
F 2 "" H 5900 2350 50  0001 C CNN
F 3 "" H 5900 2350 50  0001 C CNN
	1    5900 2350
	1    0    0    -1  
$EndComp
$Comp
L 74xx:74LS126 U5
U 2 1 61D0DBBE
P 5550 3550
F 0 "U5" H 5550 3867 50  0000 C CNN
F 1 "74HC126" H 5550 3776 50  0000 C CNN
F 2 "Package_SO:SOIC-14_3.9x8.7mm_P1.27mm" H 5550 3550 50  0001 C CNN
F 3 "http://www.ti.com/lit/gpn/sn74LS126" H 5550 3550 50  0001 C CNN
	2    5550 3550
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R2
U 1 1 61D85DFC
P 5550 3900
F 0 "R2" H 5609 3946 50  0000 L CNN
F 1 "10K" H 5609 3855 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 5550 3900 50  0001 C CNN
F 3 "~" H 5550 3900 50  0001 C CNN
	1    5550 3900
	1    0    0    -1  
$EndComp
Wire Wire Line
	5550 4000 5900 4000
$Comp
L power:+3.3V #PWR012
U 1 1 61D85E03
P 5900 4000
F 0 "#PWR012" H 5900 3850 50  0001 C CNN
F 1 "+3.3V" H 5915 4173 50  0000 C CNN
F 2 "" H 5900 4000 50  0001 C CNN
F 3 "" H 5900 4000 50  0001 C CNN
	1    5900 4000
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R3
U 1 1 61D87FB9
P 5550 5500
F 0 "R3" H 5609 5546 50  0000 L CNN
F 1 "10K" H 5609 5455 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 5550 5500 50  0001 C CNN
F 3 "~" H 5550 5500 50  0001 C CNN
	1    5550 5500
	1    0    0    -1  
$EndComp
Wire Wire Line
	5550 5600 5900 5600
$Comp
L power:+3.3V #PWR013
U 1 1 61D87FC0
P 5900 5600
F 0 "#PWR013" H 5900 5450 50  0001 C CNN
F 1 "+3.3V" H 5915 5773 50  0000 C CNN
F 2 "" H 5900 5600 50  0001 C CNN
F 3 "" H 5900 5600 50  0001 C CNN
	1    5900 5600
	1    0    0    -1  
$EndComp
$Comp
L Connector_Generic:Conn_01x04 J10
U 1 1 61D89B0E
P 7950 4150
F 0 "J10" H 8030 4142 50  0000 L CNN
F 1 "Conn_01x04" H 8030 4051 50  0000 L CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x04_P2.54mm_Vertical" H 7950 4150 50  0001 C CNN
F 3 "~" H 7950 4150 50  0001 C CNN
	1    7950 4150
	1    0    0    -1  
$EndComp
Wire Wire Line
	7550 1900 7550 4050
Wire Wire Line
	7550 4050 7750 4050
Wire Wire Line
	5850 1900 7550 1900
Wire Wire Line
	7400 3550 7400 4150
Wire Wire Line
	7400 4150 7750 4150
Wire Wire Line
	5850 3550 7400 3550
Wire Wire Line
	7400 5150 7400 4250
Wire Wire Line
	7400 4250 7750 4250
Wire Wire Line
	5850 5150 7400 5150
Wire Wire Line
	7550 4350 7750 4350
Wire Wire Line
	5850 6800 6850 6800
Wire Wire Line
	4550 3150 4800 3150
$Comp
L power:+3.3V #PWR05
U 1 1 61D13C51
P 4250 3750
F 0 "#PWR05" H 4250 3600 50  0001 C CNN
F 1 "+3.3V" H 4265 3923 50  0000 C CNN
F 2 "" H 4250 3750 50  0001 C CNN
F 3 "" H 4250 3750 50  0001 C CNN
	1    4250 3750
	-1   0    0    1   
$EndComp
Wire Wire Line
	4550 2950 4800 2950
Wire Wire Line
	4550 3550 5250 3550
$Comp
L power:GND #PWR09
U 1 1 61D17EAB
P 4150 2650
F 0 "#PWR09" H 4150 2400 50  0001 C CNN
F 1 "GND" H 4200 2500 50  0000 C CNN
F 2 "" H 4150 2650 50  0001 C CNN
F 3 "" H 4150 2650 50  0001 C CNN
	1    4150 2650
	-1   0    0    1   
$EndComp
Wire Wire Line
	3850 3350 3300 3350
Wire Wire Line
	3850 3450 3300 3450
Wire Wire Line
	3850 3550 3300 3550
Wire Wire Line
	3850 3250 3300 3250
Wire Wire Line
	4550 3350 4800 3350
Wire Wire Line
	4550 3250 4800 3250
$Comp
L 74xx:74HC4051 U3
U 1 1 61D08BAC
P 4250 3250
F 0 "U3" H 4650 2700 50  0000 C CNN
F 1 "74HC4051" H 3950 2700 50  0000 C CNN
F 2 "Package_SO:SOIC-16_3.9x9.9mm_P1.27mm" H 4250 2850 50  0001 C CNN
F 3 "http://www.ti.com/lit/ds/symlink/cd74hc4051.pdf" H 4250 2850 50  0001 C CNN
	1    4250 3250
	-1   0    0    1   
$EndComp
$Comp
L power:GND #PWR02
U 1 1 61DF8BC0
P 1200 2150
F 0 "#PWR02" H 1200 1900 50  0001 C CNN
F 1 "GND" H 1205 1977 50  0000 C CNN
F 2 "" H 1200 2150 50  0001 C CNN
F 3 "" H 1200 2150 50  0001 C CNN
	1    1200 2150
	1    0    0    -1  
$EndComp
Wire Wire Line
	1650 1900 1200 1900
Wire Wire Line
	5550 7050 5550 7400
Text Label 1300 1900 0    50   ~ 0
MCLK_EN
Entry Wire Line
	1800 1900 1700 1800
Entry Wire Line
	1700 1700 1800 1800
Entry Wire Line
	1700 1600 1800 1700
Entry Wire Line
	1700 1500 1800 1600
Wire Wire Line
	1200 1500 1700 1500
Wire Wire Line
	1700 1600 1200 1600
Wire Wire Line
	1700 1700 1200 1700
Wire Wire Line
	1700 1800 1200 1800
Text Label 1300 1800 0    50   ~ 0
~SRC_EN
Text Label 1300 1700 0    50   ~ 0
S2
Text Label 1300 1600 0    50   ~ 0
S1
Text Label 1300 1500 0    50   ~ 0
S0
$Comp
L Connector_Generic:Conn_01x07 J1
U 1 1 61E1DF42
P 1000 1700
F 0 "J1" H 1250 1150 50  0000 C CNN
F 1 "Conn_01x07" H 1100 1250 50  0000 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x07_P2.54mm_Vertical" H 1000 1700 50  0001 C CNN
F 3 "~" H 1000 1700 50  0001 C CNN
	1    1000 1700
	-1   0    0    1   
$EndComp
Wire Wire Line
	1200 1400 1250 1400
Wire Wire Line
	1250 1400 1250 900 
$Comp
L power:+3.3V #PWR01
U 1 1 61E2259C
P 1250 800
F 0 "#PWR01" H 1250 650 50  0001 C CNN
F 1 "+3.3V" H 1265 973 50  0000 C CNN
F 2 "" H 1250 800 50  0001 C CNN
F 3 "" H 1250 800 50  0001 C CNN
	1    1250 800 
	1    0    0    -1  
$EndComp
Wire Wire Line
	1200 2000 1200 2100
$Comp
L power:PWR_FLAG #FLG01
U 1 1 61E2B88C
P 1250 900
F 0 "#FLG01" H 1250 975 50  0001 C CNN
F 1 "PWR_FLAG" V 1250 1028 50  0000 L CNN
F 2 "" H 1250 900 50  0001 C CNN
F 3 "~" H 1250 900 50  0001 C CNN
	1    1250 900 
	0    1    1    0   
$EndComp
Connection ~ 1250 900 
Wire Wire Line
	1250 900  1250 800 
Wire Bus Line
	4900 600  1800 600 
Wire Wire Line
	6850 6800 6850 5600
Wire Wire Line
	6850 5600 7550 5600
Wire Wire Line
	7550 5600 7550 4350
$Comp
L Connector_Generic:Conn_01x04 J6
U 1 1 61E89987
P 2000 4200
F 0 "J6" H 1918 3775 50  0000 C CNN
F 1 "Conn_01x04" H 1918 3866 50  0000 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x04_P2.54mm_Vertical" H 2000 4200 50  0001 C CNN
F 3 "~" H 2000 4200 50  0001 C CNN
	1    2000 4200
	-1   0    0    1   
$EndComp
Wire Wire Line
	2200 4000 2550 4000
Wire Wire Line
	2200 4100 2550 4100
Wire Wire Line
	2200 4200 2550 4200
Wire Wire Line
	2200 4300 2550 4300
Text Label 2200 4000 0    50   ~ 0
LRCLK_4
Text Label 2200 4100 0    50   ~ 0
BCLK_4
Text Label 2200 4200 0    50   ~ 0
DATA_4
Text Label 2200 4300 0    50   ~ 0
MCLK_4
$Comp
L Connector_Generic:Conn_01x04 J7
U 1 1 61E89999
P 2000 4950
F 0 "J7" H 1918 4525 50  0000 C CNN
F 1 "Conn_01x04" H 1918 4616 50  0000 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x04_P2.54mm_Vertical" H 2000 4950 50  0001 C CNN
F 3 "~" H 2000 4950 50  0001 C CNN
	1    2000 4950
	-1   0    0    1   
$EndComp
Wire Wire Line
	2200 4750 2550 4750
Wire Wire Line
	2200 4850 2550 4850
Wire Wire Line
	2200 4950 2550 4950
Wire Wire Line
	2200 5050 2550 5050
Text Label 2200 4750 0    50   ~ 0
LRCLK_5
Text Label 2200 4850 0    50   ~ 0
BCLK_5
Text Label 2200 4950 0    50   ~ 0
DATA_5
Text Label 2200 5050 0    50   ~ 0
MCLK_5
$Comp
L Connector_Generic:Conn_01x04 J8
U 1 1 61E899AB
P 2000 5750
F 0 "J8" H 1918 5325 50  0000 C CNN
F 1 "Conn_01x04" H 1918 5416 50  0000 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x04_P2.54mm_Vertical" H 2000 5750 50  0001 C CNN
F 3 "~" H 2000 5750 50  0001 C CNN
	1    2000 5750
	-1   0    0    1   
$EndComp
Wire Wire Line
	2200 5550 2550 5550
Wire Wire Line
	2200 5650 2550 5650
Wire Wire Line
	2200 5750 2550 5750
Wire Wire Line
	2200 5850 2550 5850
Text Label 2200 5550 0    50   ~ 0
LRCLK_6
Text Label 2200 5650 0    50   ~ 0
BCLK_6
Text Label 2200 5750 0    50   ~ 0
DATA_6
Text Label 2200 5850 0    50   ~ 0
MCLK_6
$Comp
L Connector_Generic:Conn_01x04 J9
U 1 1 61E899BD
P 2000 6500
F 0 "J9" H 1918 6075 50  0000 C CNN
F 1 "Conn_01x04" H 1918 6166 50  0000 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x04_P2.54mm_Vertical" H 2000 6500 50  0001 C CNN
F 3 "~" H 2000 6500 50  0001 C CNN
	1    2000 6500
	-1   0    0    1   
$EndComp
Wire Wire Line
	2200 6300 2550 6300
Wire Wire Line
	2200 6400 2550 6400
Wire Wire Line
	2200 6500 2550 6500
Wire Wire Line
	2200 6600 2550 6600
Text Label 2200 6300 0    50   ~ 0
LRCLK_7
Text Label 2200 6400 0    50   ~ 0
BCLK_7
Text Label 2200 6500 0    50   ~ 0
DATA_7
Text Label 2200 6600 0    50   ~ 0
MCLK_7
Text Label 3400 1500 0    50   ~ 0
LRCLK_4
Text Label 3400 1400 0    50   ~ 0
LRCLK_5
Text Label 3400 1300 0    50   ~ 0
LRCLK_6
Text Label 3400 1200 0    50   ~ 0
LRCLK_7
Wire Wire Line
	3850 2850 3300 2850
Wire Wire Line
	3850 2950 3300 2950
Wire Wire Line
	3850 3050 3300 3050
Wire Wire Line
	3850 3150 3300 3150
Text Label 3400 3150 0    50   ~ 0
BCLK_4
Text Label 3400 3050 0    50   ~ 0
BCLK_5
Text Label 3400 2950 0    50   ~ 0
BCLK_6
Text Label 3400 2850 0    50   ~ 0
BCLK_7
Wire Wire Line
	3850 4750 3300 4750
Wire Wire Line
	3850 4650 3300 4650
Wire Wire Line
	3850 4550 3300 4550
Wire Wire Line
	3850 4450 3300 4450
Text Label 3400 4750 0    50   ~ 0
DATA_4
Text Label 3400 4650 0    50   ~ 0
DATA_5
Text Label 3400 4550 0    50   ~ 0
DATA_6
Text Label 3400 4450 0    50   ~ 0
DATA_7
Wire Wire Line
	3800 6100 3300 6100
Wire Wire Line
	3800 6200 3300 6200
Wire Wire Line
	3800 6300 3300 6300
Wire Wire Line
	3800 6400 3300 6400
Text Label 3400 6400 0    50   ~ 0
MCLK_4
Text Label 3400 6300 0    50   ~ 0
MCLK_5
Text Label 3400 6200 0    50   ~ 0
MCLK_6
Text Label 3400 6100 0    50   ~ 0
MCLK_7
Wire Notes Line
	700  3700 2750 3700
Wire Notes Line
	2750 3700 2750 7050
Wire Notes Line
	2750 7050 700  7050
Wire Notes Line
	700  7050 700  3700
Text Notes 1300 7000 0    50   ~ 0
I2S Source Inputs\n
Wire Notes Line
	750  2550 750  1050
Wire Notes Line
	750  1050 2800 1050
Wire Notes Line
	2800 1050 2800 2550
Wire Notes Line
	2800 2550 750  2550
Text Notes 1400 2500 0    50   ~ 0
Source Selection
Wire Notes Line
	7700 3750 7700 4600
Wire Notes Line
	7700 4600 9000 4600
Wire Notes Line
	9000 4600 9000 3750
Wire Notes Line
	9000 3750 7700 3750
Text Notes 8000 4550 0    50   ~ 0
Mulitplexed I2S Output\n
$Comp
L 74xx:74LS126 U5
U 5 1 61D0FA99
P 6800 1250
F 0 "U5" H 7030 1296 50  0000 L CNN
F 1 "74HC126" H 7030 1205 50  0000 L CNN
F 2 "Package_SO:SOIC-14_3.9x8.7mm_P1.27mm" H 6800 1250 50  0001 C CNN
F 3 "http://www.ti.com/lit/gpn/sn74LS126" H 6800 1250 50  0001 C CNN
	5    6800 1250
	0    -1   -1   0   
$EndComp
Wire Wire Line
	4550 1500 4800 1500
$Comp
L power:+3.3V #PWR04
U 1 1 61D1363C
P 4250 2100
F 0 "#PWR04" H 4250 1950 50  0001 C CNN
F 1 "+3.3V" H 4265 2273 50  0000 C CNN
F 2 "" H 4250 2100 50  0001 C CNN
F 3 "" H 4250 2100 50  0001 C CNN
	1    4250 2100
	-1   0    0    1   
$EndComp
Wire Wire Line
	4550 1300 4800 1300
Wire Wire Line
	3850 1400 3300 1400
Wire Wire Line
	3850 1200 3300 1200
Wire Wire Line
	4550 1900 5250 1900
Wire Wire Line
	3850 1300 3300 1300
Wire Wire Line
	3850 1700 3300 1700
Wire Wire Line
	3850 1800 3300 1800
Wire Wire Line
	3850 1900 3300 1900
Wire Wire Line
	3850 1600 3300 1600
Wire Wire Line
	4550 1700 4800 1700
Wire Wire Line
	4550 1600 4800 1600
Wire Wire Line
	3850 1500 3300 1500
$Comp
L 74xx:74HC4051 U2
U 1 1 61D085EF
P 4250 1600
F 0 "U2" H 4600 1050 50  0000 C CNN
F 1 "74HC4051" H 3950 1050 50  0000 C CNN
F 2 "Package_SO:SOIC-16_3.9x9.9mm_P1.27mm" H 4250 1200 50  0001 C CNN
F 3 "http://www.ti.com/lit/ds/symlink/cd74hc4051.pdf" H 4250 1200 50  0001 C CNN
	1    4250 1600
	-1   0    0    1   
$EndComp
$Comp
L power:GND #PWR08
U 1 1 61D15C26
P 4150 1000
F 0 "#PWR08" H 4150 750 50  0001 C CNN
F 1 "GND" H 4200 850 50  0000 C CNN
F 2 "" H 4150 1000 50  0001 C CNN
F 3 "" H 4150 1000 50  0001 C CNN
	1    4150 1000
	-1   0    0    1   
$EndComp
$Comp
L power:GND #PWR0101
U 1 1 61F1A859
P 4250 1000
F 0 "#PWR0101" H 4250 750 50  0001 C CNN
F 1 "GND" H 4200 850 50  0000 C CNN
F 2 "" H 4250 1000 50  0001 C CNN
F 3 "" H 4250 1000 50  0001 C CNN
	1    4250 1000
	-1   0    0    1   
$EndComp
$Comp
L power:GND #PWR0102
U 1 1 61F282DF
P 4250 2650
F 0 "#PWR0102" H 4250 2400 50  0001 C CNN
F 1 "GND" H 4200 2500 50  0000 C CNN
F 2 "" H 4250 2650 50  0001 C CNN
F 3 "" H 4250 2650 50  0001 C CNN
	1    4250 2650
	-1   0    0    1   
$EndComp
$Comp
L power:GND #PWR0103
U 1 1 61F34AF9
P 4250 4250
F 0 "#PWR0103" H 4250 4000 50  0001 C CNN
F 1 "GND" H 4200 4100 50  0000 C CNN
F 2 "" H 4250 4250 50  0001 C CNN
F 3 "" H 4250 4250 50  0001 C CNN
	1    4250 4250
	-1   0    0    1   
$EndComp
$Comp
L power:GND #PWR0104
U 1 1 61F40D9B
P 4100 5900
F 0 "#PWR0104" H 4100 5650 50  0001 C CNN
F 1 "GND" H 4150 5750 50  0000 C CNN
F 2 "" H 4100 5900 50  0001 C CNN
F 3 "" H 4100 5900 50  0001 C CNN
	1    4100 5900
	-1   0    0    1   
$EndComp
$Comp
L power:PWR_FLAG #FLG0101
U 1 1 61F4D8AC
P 1200 2100
F 0 "#FLG0101" H 1200 2175 50  0001 C CNN
F 1 "PWR_FLAG" V 1200 2228 50  0000 L CNN
F 2 "" H 1200 2100 50  0001 C CNN
F 3 "~" H 1200 2100 50  0001 C CNN
	1    1200 2100
	0    1    1    0   
$EndComp
Connection ~ 1200 2100
Wire Wire Line
	1200 2100 1200 2150
Wire Wire Line
	1650 1900 1650 2000
Wire Wire Line
	1650 2000 2000 2000
Wire Wire Line
	1250 5750 1600 5750
Wire Bus Line
	1800 600  1800 1900
Wire Bus Line
	4900 600  4900 7300
$EndSCHEMATC
