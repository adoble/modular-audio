# Modular Audio Project - Amplifier Module

An class D amplifier module that can select other digital sources (I2S, S/PDIF).

## System Diagram

### Functional Components

```mermaid
graph LR;

main_ctrl["Main Controller"] --- i2sm[I2SMultiplexor];
main_ctrl --- display_driver[Display Driver] --- Display;
i2sm --- dsp["Digital Signal <br> Processor"] --- amp[Amplifier];
style dsp stroke-dasharray: 5 5;


subgraph sources [Sources]
    cd{{CD}};
    dvd{{DVD}};
    aux{{Aux In}};
end

airlink["Airlink/Bluetooth <br> Module"] -- I2S --- i2sm
cd -- I2S  --- i2sm;
dvd -- SPDIF --- spdif[SPDIF Module] -- I2S --- i2sm;
aux --- adc[Analog to Digital Converter] -- I2S --- i2sm;

amp --- left((Left <br> Speaker)) 
amp --- right((Right <br> Speaker)) 


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
    reg_3v3 --- pwr_3v3{3V};

    Battery --- pwr_on_off(("Power <br> On/Off <br> Switch")) --- Relay; 


   
```

### Key

```mermaid
   graph LR;

   subgraph Key 
        transducer((Transducer)) ---  pwr_net{Power Net} --- module[Module] --- ext{{External}};
   end 

   %% Render links invisible
   linkStyle 0 stroke:none
   linkStyle 1 stroke:none
   linkStyle 2 stroke:none
   
   



```

<!-- ![System Diagram](./docs/img/Modular-Amplifier.png) -->

