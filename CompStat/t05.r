########################################################
# Завдання №5
########################################################

harmonic_series <- function(alpha, delta=1e-6, m=10000){

    if (alpha >= -1){
        stop("Порушені умови збіжності ряду.")
    }
    
    iter <- 1
    prev_sum <- 0
    result <- 0

    # Починаємо цикл. В R-і немає циклу do-while, тому окремо забезпечимо
    # першу ітерацію.
    while((iter == 1) || (abs(result - prev_sum) >= delta)){
        if (iter > m){
            stop("Перевищена кількість допустимих ітерацій.")
        }

        prev_sum <- result 
        result <- result + iter ^ alpha     
        iter <- iter + 1
    }

    print(result)
}

harmonic_series(-2, 1e-8)
