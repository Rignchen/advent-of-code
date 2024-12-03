# Advent of code, a different language every day

I asked 4 of my friends to give me a list of languages, the size of the list was randomly chosen between 2 and 12 except for the last one wich was 24 - the allready chosen languages.\
Here's the list of languages I got:

[LTHC](https://github.com/LTHCTheMaster):
1. F#
2. D
3. Julia
4. Objective-C
5. Haxe
6. Groovy
7. Scala
8. Swift\

[Dwesh](https://github.com/dwesh163):

9. Go
10. Ruby
11. JavaScript
12. Java\

[Epsilon](https://github.com/e-psi-lon):

13. Java
14. C
15. C++
16. Kotlin
17. JavaScript
18. Python
19. COBOL
20. Fortran
21. C#
22. Scala\

[Stoupy](https://github.com/Stoupy51):

23. C
24. Python
25. PHP

I will be using docker to run the code, so I don't have to install all the languages on my machine.\
That way it will also be easy for you to run these scripts on your machine if you want to.\
Here's the list of docker images I will be using:
- Python: python:3.12.3
- C: gcc:12.3.0-bookworm
- Java: openjdk:23-ea-21-oracle
- Javascript: node:22.1.0-bullseye
- Scala: sbtscala/scala-sbt:eclipse-temurin-alpine-22_36_1.10.0_3.4.1
- C#: esolang/csharp-dotnet:2.4.0
- Fortran: gcc:12.3.0-bookworm
- Cobol: esolang/cobol:2.4.0
- Kotlin: esolang/kotlin:2.4.0
- C++: gcc:12.3.0-bookworm
- Ruby: ruby:3.3.1-bullseye
- Go: golang:1.22.3-alpine3.19
- Swift: swift:6.0.2
- Groovy: groovy:jdk21
- Haxe: haxe:4.3.4-buster
- Objective C: custom docker image, see [Dockerfile](./04_Objective-C/Dockerfile)
- Julia: julia:1.6.7-bullseye
- D: custom docker image, see [Dockerfile](./02_D/Dockerfile)
- F#: mcr.microsoft.com/dotnet/sdk:8.0-alpine
- Php: php:8.3.7-fpm-alpine

I want to take a year of advent of code that I've never done before, so I'll be going with [2020](https://adventofcode.com/2020).\
My folder structure will be as follows:
```
.
├── day_language
│   ├── Dockerfile
│   ├── .gitignore
│   └── src
│       ├── input.txt
│       ├── example.txt
│       ├── script_1
│       └── script_2
├── LICENSE.md (GLWTS)
└── README.md (this file)
```

