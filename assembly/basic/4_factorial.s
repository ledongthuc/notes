#PURPOSE - Given a number, this program computes the
#          factorial.  For example,
#          4 is 4 * 3 * 2 * 1, or 24, and so on.
#
#This program shows how to call a function recursively.
.section .data
#This program has no global data
.section .text
.globl _start
.globl factorial #this is unneeded unless we want to share
                  #this function among other programs
_start:
pushl $4         #The factorial of 4
call  factorial  #run the factorial function
addl  $4, %esp   #Scrubs the parameter that was pushed on
                 #the stack
movl  %eax, %ebx #factorial returns the answer in %eax, but
                 #we want it in %ebx to send it as our exit status
movl  $1, %eax   #call the kernel’s exit function
int   $0x80

#This is the actual function definition
.type factorial, @function
factorial:
pushl %ebp           # use to restore later
movl  %esp, %ebp     # copy stack pointer to ebp and modify on it
movl  8(%ebp), %eax  # get param
cmpl $1, %eax        # check factorial param is 1 to break the loop
je end_factorial
decl  %eax           # If not,  continue call factorial with decrease
pushl %eax
call  factorial
movl  8(%ebp), %ebx  # Get final value from factorial mul with original param
imull %ebx, %eax

end_factorial:
movl  %ebp, %esp
popl  %ebp
ret
