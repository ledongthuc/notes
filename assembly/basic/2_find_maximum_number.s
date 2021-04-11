#PURPOSE:  Find maximum number 
#
#INPUT:    none
#
#OUTPUT:   returns a status code.  This can be viewed by typing echo $? after running the program
#
#VARIABLES:
#          %edi hold index of data item
#          %ebx largest data item found
#          %eax current data item
#DATA:
#          data_items contains the item data.  A 0 is used to terminate the data
#
.section .data
data_items:
 .long 3,67,34,222,45,75,54,34,44,33,22,11,66,0

.section .text
.globl _start
_start:
  movl $0, %edi                       # move 0 to registry %edi
  movl data_items(,%edi,4), %eax      # load first item of data to %eax
  movl %eax, %ebx                     # move fist item to ebx (latest item)

start_loop:
  cmpl $0, %eax                       # check if eax is 0 (end)
  je loop_exit
  incl %edi                           # increase index
  movl data_items(,%edi,4), %eax      # load next item
  cmpl %ebx, %eax                      # compare next item and curren max item
  jle start_loop                      # loop back to start_loop if ebx < eax
  movl %eax, %ebx                     # otherwise, move %ebx to ebx current max number
  jmp start_loop

loop_exit:
  movl $1, %eax
  int  $0x80
