# Watchdog


* Watchdog is a Hardware device arms to reset the machine if the machine does not send PING to watchdog after a period of time.

* Watchdog holds a Counter Register

    * Counter initialy is set to a value (e.g: 600 sec)

    * Counter is decreased over time

    * If Counter = 0, Watchdog trigger machine to reboot

    * When machine send PING to Watchdog, Counter is reloaded (set back to 600 sec)


    ![watchdog](images/watchdog.dio.svg)



* When watchdog kernel module is loaded, it will check if Watchdog device is running or not?

    * E.g:
    [iTCO_wdt_probe](https://github.com/torvalds/linux/blob/38fec10eb60d687e30c8c6b5420d86e8149f7557/drivers/watchdog/iTCO_wdt.c#L593 "/v6.14/drivers/watchdog/iTCO_wdt.c")

    ```C
    static int iTCO_wdt_probe(struct platform_device *pdev) {
        ...

        if (!iTCO_wdt_set_running(p)) {
            /*
            * If the watchdog was not running set NO_REBOOT now to
            * prevent later reboots.
            */
            p->update_no_reboot_bit(p->no_reboot_priv, true);
        }
        
        ...
    }
    ```


* Use PING from Kernel via: *CONFIG_WATCHDOG_HANDLE_BOOT_ENABLED*

    * Some Linux distro will force to has Watchdog running after boot.

    * So, it requires PING to keep the COUNTER away from zero

    * PING can come from process in UserSpace

    * OR, PING from KERNEL via *CONFIG_WATCHDOG_HANDLE_BOOT_ENABLED*:

        * The config get kernel to send ping to Watchdog after boot.

        * Kernel will stop send PING when UserSpace starts to send it's first PING.