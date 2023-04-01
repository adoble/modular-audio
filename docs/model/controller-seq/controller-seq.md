# Controller Sequence Diagram

```mermaid

sequenceDiagram

participant system
participant [task]init
participant [task]select_source
participant [struct]SelectSourceDriver
participant [struct]Source
participant [struct]I2SMultiplexerDriver


[task]init->>+SelectSourceDriver: new
SelectSourceDriver->>-[task]init: select_source_driver

system-->>[task]select_source :  interrupt on source change

[task]select_source->>+SelectSourceDriver : next_source

SelectSourceDriver->>-[task]select_source : source
[task]select_source->>[struct]Source : activate

[struct]Source->>[struct]I2SMultiplexerDriver : set_source











```
