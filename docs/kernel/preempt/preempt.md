# Preempt


* There are 3 Preempt mode

    * NONE : no preempt

    * Voluntary : task voluntarily gives up CPU to other task

    * Full : task is forced to give up CPU


* There are 2 approachs to config preempt mode

    * Static:
        * Set mode in COMPILE time, and can not change after that
    
    * Dynamic:
        * *CONFIG_PREEMPT_DYNAMIC = y*
        * Chose a default mode in COMPILE time
        * Can change mode on runtime via boot params


![preempt](images/preempt.dio.svg)
