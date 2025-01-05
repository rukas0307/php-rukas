<?php

# php -d extension=./target/debug/libphp_rukas.so examples/network.php

print_r(trace());
echo "\n";
echo trace("country");
echo "\n";
if(test_net()){
    echo "We are able to connect to the internet!";
}
