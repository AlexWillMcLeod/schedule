# Schedule

Secondary schools spend many months sorting students into classes for the new year, and there isn't an effective automated tool for doing so efficiently and effectively. Schedule's objective as a project is to simplify this entire process, by simply letting school administration enter basic information about the classrooms, students, and and the available subjects, and have Schedule create an optimised schedule without conflict, within milliseconds (as opposed to months).

As of now schedule is a simple library that will take information about students, departments, and subjects and will sort students into different classes within non-overlapping slots. We use an algorithm for this sorting, that attempts to minimise conflicts. For example:

Let's create a random school with 100 students, 8 subjects, 140 classrooms and 20 different departments, then sort it in place.

```
  let mut high_school = random::random_schedule(100, 8, 140, 20);
  high_school.sort();
```

The schedule is now sorted with classes added to different slots. Printing this out:

```
Slot: 0
  Class:
    Subject: Subject 5
    Department: Department 0
    Students:
      Person 0 (id=0)
      Person 2 (id=2)
      Person 6 (id=6)
      Person 8 (id=8)
      Person 10 (id=10)
      Person 19 (id=19)
      Person 20 (id=20)
      Person 25 (id=25)
      Person 38 (id=38)
      Person 42 (id=42)
      Person 45 (id=45)
      Person 47 (id=47)
      Person 48 (id=48)
      Person 50 (id=50)
      Person 51 (id=51)
      Person 53 (id=53)
      Person 55 (id=55)
      Person 59 (id=59)
      Person 60 (id=60)
      Person 62 (id=62)
      Person 63 (id=63)
      Person 65 (id=65)
      Person 68 (id=68)
      Person 76 (id=76)
      Person 80 (id=80)
      Person 86 (id=86)
      Person 89 (id=89)
      Person 90 (id=90)
      Person 97 (id=97)
      Person 99 (id=99)

  Class:
    Subject: Subject 7
    Department: Department 3
    Students:
      Person 1 (id=1)
      Person 3 (id=3)
      Person 4 (id=4)
      Person 5 (id=5)

 ...(cut out for brevity)

Slot: 4
  Class:
    Subject: Subject 0
    Department: Department 3
    Students:
      Person 0 (id=0)
      Person 1 (id=1)
      Person 2 (id=2)
      Person 3 (id=3)
      Person 4 (id=4)
      Person 7 (id=7)
      Person 8 (id=8)
      Person 9 (id=9)
      Person 10 (id=10)
      Person 12 (id=12)
      Person 14 (id=14)
      Person 17 (id=17)
      Person 18 (id=18)
      Person 20 (id=20)
      Person 21 (id=21)
      Person 22 (id=22)
      Person 24 (id=24)
      Person 26 (id=26)
      Person 29 (id=29)
      Person 31 (id=31)
      Person 32 (id=32)
      Person 33 (id=33)
      Person 35 (id=35)
      Person 36 (id=36)
      Person 37 (id=37)
      Person 38 (id=38)
      Person 67 (id=67)
      Person 79 (id=79)
      Person 80 (id=80)
      Person 97 (id=97)



  Class:
    Subject: Subject 3
    Department: Department 7
    Students:
      Person 23 (id=23)
      Person 25 (id=25)
      Person 34 (id=34)
      Person 39 (id=39)
      Person 41 (id=41)
      Person 42 (id=42)
      Person 45 (id=45)
      Person 46 (id=46)
      Person 51 (id=51)
      Person 52 (id=52)
      Person 53 (id=53)
      Person 56 (id=56)
      Person 60 (id=60)
      Person 63 (id=63)
      Person 65 (id=65)
      Person 72 (id=72)
      Person 75 (id=75)
      Person 77 (id=77)
      Person 81 (id=81)
      Person 89 (id=89)
      Person 99 (id=99)

  Class:
    Subject: Subject 1
    Department: Department 6
    Students:
      Person 47 (id=47)
      Person 48 (id=48)
      Person 49 (id=49)
      Person 55 (id=55)
      Person 57 (id=57)
      Person 58 (id=58)
      Person 59 (id=59)
      Person 62 (id=62)
      Person 64 (id=64)
      Person 66 (id=66)
      Person 68 (id=68)
      Person 69 (id=69)
      Person 74 (id=74)
      Person 76 (id=76)
      Person 78 (id=78)
      Person 82 (id=82)
      Person 83 (id=83)
      Person 84 (id=84)
      Person 85 (id=85)
      Person 91 (id=91)

```

Schedule will add every student into every subject it can, make sure that there is physical space and time available for a class, and then remove the classes that are too small. Yet to be added post processing should assign every class a teacher, a specific classroom and periods instead of slots.

In the future, Schedule aims to be web app where school administration set up a school, and students login and select their subjects, where Schedule then sorts based off of this information, and shows them all their timetable within the schedule app.
