# Tunnel


### Encapsulate Tunnel

* Just a simple interface which:

    * Wrap a package into a bigger package

    ![encapsulate](images/encapsulate.dio.svg)


    * The purpose is to change the *source IP* and *destination IP*

        * The bigger package will enter the linux routing subsystem (ipfilter, iptable, ...)
        
        * and receive a proper route coresponse with new *source IP* and *destination IP*


* Note:

    * Some people thing creating a tunnel is kind of creating a new Layer 3 path.
    Because when they type *ifconfig* and see tunnel interface same as eth0 interface

    * But, it's not true

    * Tunnel simply changes the src/des IP. And then, kernel will find a different route for the new bigger package