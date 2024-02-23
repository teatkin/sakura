# Sakura :cherry_blossom:

Sakura is a Real Time Operating System (RTOS) designed for microcontrollers, written in
Rust as a learning project. It is a soft real-time operating system, mostly due to the
fact that my experience in this area is limited to a rather minimally featured monolithic
kernel based on Unix and a bit of time spent working with FreeRTOS on a hobby project in
the past.

## What Sakura is
* A hobby project
* A learning exercise
* A soft real-time target
* Developed for a specific microcontroller to begin with (the Espressif ESP32C3)

## What Sakura is not
* An OS you should put anywhere near a project you care about working
* An OS you expect timely support for (If you want support, you work to my time schedule)
* An OS that is well engineered - I know 1/100th as much about OS development as I'd
like to know.

I hope some of you will learn something from this repo, as I have learned from the repos
and people who have come before me. Some people I would specifically like to mention as
having had a profound influence on me in my desire to make this project work include:
* [Cliff Biffle](https://github.com/cbiffle)
* [Stefan Lüthi](https://gitlab.com/bern-rtos/bern-rtos)
* [Philipp Oppermann](https://os.phil-opp.com)
* [Jeremy Soller](https://github.com/jackpot51)

## Footnote
I have set myself the goal of having an RTOS that can manage a task that blinks an LED on
the dev board I have bought by the time I go to EMFCamp in May 2024.
