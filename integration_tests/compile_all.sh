for i in *.asm6502; do
    [ -f "$i" ] || break

    # rm "${i%.*}.hex"
    cp $i "${i%.*}.bin" 
     ./assemble -o "${i%.*}.bin" >> nul
    if [ $? -eq 0 ]; then 
        echo -e "\033[1;32mCOMPILED\033[0m: '${i%.*}.asm6502'"
    else 
        echo -e "\033[0;31mFAILED TO COMPILE\033[0m: '${i%.*}.asm6502'"
    fi

    # cat "${i%.*}.hex" | cut -c 2- > "${i%.*}.hex"
    # xxd -r -p "${i%.*}.hex" > tmp_output
    # cp tmp_output "${i%.*}.bin"
    # rm tmp_output
done
