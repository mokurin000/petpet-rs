set xlabel "speed"
set ylabel "cpu time"
set title "speed - cpu time"
set xrange[0:31]
set xtics 1,1,30
plot "data.dat" with linespoints
pause mouse close
