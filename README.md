# Modular Audio Project - Amplifier Module

An class D amplifier module that can select other digital sources (I2S, S/PDIF).

## System Diagram

### Functional Components

```mermaid
graph LR;

main_ctrl["Main Controller"] --- i2sm[I2SMultiplexor];
main_ctrl --- display_driver[Display Driver] --- Display;
main_ctrl -- SPI --- dsp;
i2sm -- I2S --- dsp["Digital Signal <br> Processor"] -- I2S --- amp[Amplifier];
dsp -- I2S --- toslink_out_converter[TOSLINK<br>Converter] -- "S/PDIF" --- toslink_out{{TOSLINK <br> Out}};

style dsp stroke-dasharray: 5 5;
style toslink_out_converter stroke-dasharray: 5 5;
style toslink_out stroke-dasharray: 5 5;


subgraph sources [Sources]
    cd{{CD}};
    dvd{{DVD}};
    tos_link_in{{Toslink In}}
    aux{{Aux In}};
end

airlink["Airlink/Bluetooth <br> /Internet Radio <br> Module"] -- I2S --- i2sm
cd -- I2S  --- i2sm;
dvd -- "S/PDIF" --- spdif["S/PDIF Module"] -- I2S --- i2sm;
aux --- adc[Analog to <br>Digital Converter] -- I2S --- i2sm;
tos_link_in -- "S/PDIF" --- toslink_converter[TOSLINK<br>Converter] -- I2S --- i2sm; 

amp --- left{{Left}} --- left_spk((Left <br> Speaker)) 
amp --- right{{Right}} --- right_spk((Right <br> Speaker)) 


subgraph rot_procs[Rotary Control <br> Processors]
    vol_proc[Volume <br> Processor];
    sel_proc[Select <br> Processor];
    
end

subgraph controls[Controls]
    direction RL;
    vol_knob((Volume <br> Knob));
    sel_knob((Select <br> Knob));
    sel_btns((Selection <br> buttons ...))
end

vol_knob --- vol_proc --- main_ctrl;
sel_knob --- sel_proc --- main_ctrl;
sel_btns --- main_ctrl;
```

### Power

```mermaid
    graph LR;

    net{{240V AV}};
    net ---- Relay;
    Relay --- smps[Switched Mode <br> Power Supply];
    smps ---- pwr_24v{24V};
    smps --- reg_5v[5V Regulator];
    smps --- reg_3v3[3.3V Regulator];
    reg_5v --- pwr_5v{5V};
    reg_3v3 --- pwr_3v3{3.3V};

    Battery --- pwr_on_off(("Power <br> On/Off <br> Switch")) --- Relay; 
```

### Symbols used in the Diagrams

```mermaid
graph LR;

subgraph key[Key]
    transducer((<small>Transducer<small>)) ---  pwr_net{<small>Power<br>Net<small>} --- module[<small>Module<small>] --- ext{{<small>External<br>Connection<small>}};
end
style key stroke-dasharray: 5 5, fill-opacity: 0;

%% Render links invisible
linkStyle 0 stroke:none
linkStyle 1 stroke:none
linkStyle 2 stroke:none
```

<!-- ![System Diagram](./docs/img/Modular-Amplifier.png) -->

