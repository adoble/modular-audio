![Project Status: Work In Progress](https://img.shields.io/badge/project--status-work--in--progress-orange)


# Modular Audio Project - Amplifier Module

An class D amplifier module that can select other digital sources (I2S, S/PDIF).


## System Diagram

### Functional Components

<!-- Removed the ELK renderer as causing problems - see https://github.com/mermaid-js/mermaid-cli/issues/594 -->
<!-- Old code was (first line of the mermaid description): -->
<!-- %%{init: {"flowchart": {"defaultRenderer": "elk"}} }%% -->

```mermaid
flowchart LR

main_ctrl["Main Controller"] -- Parallel Bus --- i2sm[I2S-Multiplexer];
wifi --- antenna1((Antenna <br> 1));

main_ctrl -- I2C --- dsp;
main_ctrl -- SPI+2 --- wifi["WiFi Co-Processor"]
i2sm -- I2S --- dsp["Digital Signal <br> Processor"] -- I2S --- amp[Amplifier];
dsp -- I2S --- toslink_out_converter[TOSLINK<br>Converter] -- "S/PDIF" --- toslink_out{{TOSLINK <br> Out}};


style dsp stroke-dasharray: 5 5;
style toslink_out_converter stroke-dasharray: 5 5;
style toslink_out stroke-dasharray: 5 5;


subgraph sources [Sources-In]
    cd{{CD}};
    dvd{{DVD}};
    tos_link_in{{Toslink In}}
end

subgraph source_sel_display [Source Selection Lights]
    direction LR
    cd_source_selected((CD ))
    dvd_source_selected((DVD))
    bluetooth_source_selected((BT ))
    wlan_source_selected((WLAN))
    dab_source_selected((DAB))
end

antenna2((Antenna <br> 2)) --- airlink;
airlink["Airlink/Bluetooth"] -- I2S --- i2sm
cd -- I2S  --- i2sm;
dvd -- "S/PDIF" --- spdif["S/PDIF Module"] -- I2S --- i2sm;
internet-radio["Internet Radio"] -- I2S --- i2sm

tos_link_in -- "S/PDIF" --- toslink_converter[TOSLINK<br>Converter] -- I2S --- i2sm; 
dab["DAB Radio"] -- I2S --- i2sm

amp --- left{{Left}} --- left_spk((Left <br> Speaker)) 
amp --- right{{Right}} --- right_spk((Right <br> Speaker)) 

dsp -- I2S --- sw-dac[Sub-Woofer DAC] --- sw-line-out{{Sub-Woofer<br>Line-Out}} --- active-sw((Active<br>Sub-Woofer))


vol_proc[Volume <br> Processor];
sel_source[Select <br> Source <br>];

subgraph controls[Controls]
    direction TB
    sel_source_btn((Select <br> Source Button))
    vol_knob((Volume <br> Knob))
end

sel_source --- source_sel_display
vol_knob --- vol_proc -- I2C+1--- main_ctrl;
sel_source_btn --- sel_source -- I2C+1--- main_ctrl;


%%click main_ctrl "https://github.com/adoble/modular-audio/blob/main/README.md#controller"%%
%%click i2sm "https://github.com/adoble/modular-audio/tree/main/hardware/i2s-multiplexer" "Click to see sub-repository"%%
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
    transducer((<small>Transducer<small>)) ---  pwr_net{<small>Power<br>Net<small>} --- module[<small>Hardware Module<small>] --- ext{{<small>External<br>Connection<small>}};
end
style key stroke-dasharray: 5 5, fill-opacity: 0;

%% Render links invisible
linkStyle 0 stroke:none
linkStyle 1 stroke:none
linkStyle 2 stroke:none
```

<!-- ![System Diagram](./docs/img/Modular-Amplifier.png) -->

## Functional Component Descriptions

### Controller

Work in Progress!

The controller has the following tasks:

* Select sources
* Receive input fron the volume control and adjust the volume over the DSP. 
* Receive input from the button banks. These are used to interact with the display. 
* Interface with an (Airlift) Wifi board over an extended SPI interface. This is used to download station lists and also 
to control the internet radio module.

Software is [here](/software/main-controller).

Hardware is [here](/hardware/controller).
