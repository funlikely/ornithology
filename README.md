# ornithology

This is a Rust program to solve the Programmer's Day challenge.

Ivan has created a Time Machine and plans to test it by traveling to Russia on a significant date for coders – the Day of the Programmer, which is the 256th day of the year. His time-travel range is set from the year 1700 to 2700.

 

Historically, Russia followed the Julian calendar until 1917. After a gap year in 1918 due to the calendar switch, the Gregorian calendar was adopted from 1919 onwards. In this transition year of 1918, February started on the 14th, which was counted as the 32nd day of the year.

 

In both calendar systems, February has either 28 or 29 days, with 29 days in a leap year. Leap years in the Julian calendar occur every 4 years. In the Gregorian calendar, a leap year is one that is either divisible by 400 or divisible by 4 but not 100.

 

A year is a leap year in the Gregorian calendar if it is:

1.       Divisible by 4 and

2.       Not divisible by 100 unless

3.       It is also divisible by 400.

Therefore, a year that is divisible by 100 is not a leap year unless it is also divisible by 400. This means, for example, that the year 2000 was a leap year, as it is divisible by 400, but 1900 was not, even though it is divisible by 100, because it is not divisible by 400.

 

 

Your task is to determine the date of the 256th day of any given year within the specified range, according to the calendar system Russia followed at the time. Output this date in the format of dd.mm.yyyy.

 

For instance, in 1984, a leap year in the Gregorian calendar, the 256th day is September 12, so the output for that year would be 12.09.1984.

 

Function Objective: Create a function dayOfProgrammer that returns the date of the 256th day for a given year in the format of a string "dd.mm.yyyy".

 

Input Parameters:

    year: an integer representing the year.

 

Constraints:

    The year must be between 1700 and 2700, inclusive.

 

Output Format:

    Print the date of the Day of the Programmer for the given year as "dd.mm.yyyy".

 

 

 

Input and expected output examples:

Input: 2017 Output: 13.09.2017

Input: 2016 Output: 12.09.2016

Input: 1800 Output: 12.09.1800