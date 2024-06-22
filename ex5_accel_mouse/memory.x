/* Linker script defining the memory layout of the target microcontroller */

/* Define the memory regions available on the target microcontroller */
MEMORY
{
  /* Flash memory region
   * ORIGIN: Starting address of the flash memory
   * LENGTH: Size of the flash memory in bytes
   */
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K

  /* RAM memory region
   * ORIGIN: Starting address of the RAM
   * LENGTH: Size of the RAM in bytes
   */
  RAM : ORIGIN = 0x20000000, LENGTH = 16K
}