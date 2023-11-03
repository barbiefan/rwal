#!/bin/sh
[ "${TERM:-none}" = "linux" ] && \
    printf '%b' '\e]P0{color_0 hex_code}
                 \e]P1{color_1 hex_code}
                 \e]P2{color_2 hex_code}
                 \e]P3{color_3 hex_code}
                 \e]P4{color_4 hex_code}
                 \e]P5{color_5 hex_code}
                 \e]P6{color_6 hex_code}
                 \e]P7{color_7 hex_code}
                 \e]P8{color_8 hex_code}
                 \e]P9{color_9 hex_code}
                 \e]PA{color_10 hex_code}
                 \e]PB{color_11 hex_code}
                 \e]PC{color_12 hex_code}
                 \e]PD{color_13 hex_code}
                 \e]PE{color_14 hex_code}
                 \e]PF{color_15 hex_code}
                 \ec'
