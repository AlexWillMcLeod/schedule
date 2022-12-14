# Schedule

Secondary schools spend many months sorting students into classes for the new year, and there isn't an effective automated tool for doing so efficiently and effectively. Schedule's objective as a project is to simplify this entire process, by simply letting school administration enter basic information about the classrooms, students, and and the available subjects, and have Schedule create an optimised schedule without conflict, within milliseconds (as opposed to months).

As of now schedule is a simple library that will take information about students, departments, and subjects and will sort students into different classes within non-overlapping slots. We use an algorithm for this sorting, that attempts to minimise conflicts. For example:

Let's create a random school with 100 students, 8 subjects, 140 classrooms and 20 different departments, then sort it in place.

```
  let mut high_school = random::random_schedule(100, 8, 140, 20);
  high_school.sort();
```

The schedule is now sorted with classes added to different slots. This is what the result may look like, just with most information omitted to save space:

```
Slot: 0
  Class:
    Subject: Subject 5
    Department: Department 0
    Students:
      Person 0 (id=0)
      ...
      Person 99 (id=99)

  Class:
    Subject: Subject 7
    Department: Department 3
    Students:
      Person 1 (id=1)
      ...
      Person 5 (id=5)

...

Slot: 4
  ...
  Class:
    Subject: Subject 1
    Department: Department 6
    Students:
      Person 47 (id=47)
      ...
      Person 91 (id=91)

```

Schedule will add every student into every subject it can, make sure that there is physical space and time available for a class, and then remove the classes that are too small. Yet to be added post processing should assign every class a teacher, a specific classroom and periods instead of slots.

In the future, Schedule aims to be web app where school administration set up a school, and students login and select their subjects, where Schedule then sorts based off of this information, and shows them all their timetable within the schedule app.
