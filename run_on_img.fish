for image in ./images/*
    echo "----------BEGIN EXECUTION ON $image-----------"
    set imagefilename (basename (echo $image | sed 's/\.[^.]*$//'))
    mkdir ./out/$imagefilename
    for color_file in ./color_files/*
        echo ">>> Using color specification $color_file"
        echo ">>> Executing naive test"
        set colorsfilename (basename (echo $color_file| sed 's/\.[^.]*$//'))
        perf stat -B -e cache-references,cache-misses,cycles,instructions,branches,faults cargo run --release -- -n -c $color_file -i $image -o ./out/$imagefilename/$colorsfilename\_naive.png
        echo ">>> Executing k-d tree test"
        perf stat -B -e cache-references,cache-misses,cycles,instructions,branches,faults cargo run --release -- -c $color_file -i $image -o ./out/$imagefilename/$colorsfilename.png
    end
    echo "----------END EXECUTION ON $image----------"
end
