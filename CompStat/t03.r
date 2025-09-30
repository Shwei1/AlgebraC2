########################################################
# Завдання №3
########################################################

PulpFiction <- list(
        Title = "Pulp Fiction",
        Director = "Quentin Tarantino",
        Type = "Film",
        Genre = "Crime",
        Year = 1994,
        Duration = 154,
        Characters = data.frame(
            Name = c("Vincent Vega", "Jules Winnfield", "Mia Wallace", 
            "Winston Wolfe", "Marsellus Wallace"),
            Actor = c("John Travolta", "Samuel L. Jackson", "Uma Thurman", "Harvey Keitel", "Ving Rhames"),
            Survived = c(FALSE, TRUE, TRUE, TRUE, TRUE),
            Good = c(FALSE, FALSE, FALSE, FALSE, FALSE)
        )
)

print(PulpFiction)
