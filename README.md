### Rust Weather CLI Application

#### Author: Darren Foley

#### Date: 2023-07-02

#### Description

"weather" is a rust based cli application that calls weatherapi.com and returns a daily weather summary. The application takes two parameters:

1. method - Type of weather summary you would like - ["forecast", "today"]
2. days - Number of days of forecast you would like - defaults to 1

<br>

#### Output

The report output for today only, looks like the following:

```
            [Location]: "Rathcoole", "Dublin", "Ireland"

            [Date]: "2023-07-03 19:45"

            [Summary]: "Partly cloudy"
 
            [Temperature Deg/C]: 14.0

            [Rain mm]: 0.1

            [Wind km/hr]: 11.2
```

Forecast reports look like the following:


```
--------------------------------
                [Forecast Day]: 1

                [Location]: "Rathcoole", "Dublin", "Ireland"

                [Date]: "2023-07-03"

                [Summary]: "Moderate rain"
 
                [Temperature Deg/C]: 11.4

                [Rain mm]: 17.4

                [Wind km/hr]: 27.7
                --------------------------------
                

                
                --------------------------------
                [Forecast Day]: 2

                [Location]: "Rathcoole", "Dublin", "Ireland"

                [Date]: "2023-07-04"

                [Summary]: "Moderate rain"
 
                [Temperature Deg/C]: 10.8

                [Rain mm]: 10.5

                [Wind km/hr]: 22.3
                --------------------------------
                

                
                --------------------------------
                [Forecast Day]: 3

                [Location]: "Rathcoole", "Dublin", "Ireland"

                [Date]: "2023-07-05"

                [Summary]: "Moderate rain"
 
                [Temperature Deg/C]: 12.0

                [Rain mm]: 5.0

                [Wind km/hr]: 24.1
                --------------------------------
```


