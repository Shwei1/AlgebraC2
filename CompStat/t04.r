########################################################
# Завдання №4
########################################################

stats <- function(x){
    arr <- numeric(10)
    arr[1] <- mean(x)
    arr[2] <- exp(mean(log(x)))
    arr[3] <- length(x)/sum(1/x)
    arr[4] <- median(x)
    arr[5] <- var(x)
    arr[6] <- sd(x)
    arr[7] <- mad(x)
    arr[8] <- mean(abs(x-mean(x)))
    arr[9] <- IQR(x)
    arr[10] <- diff(range(x))
        
    names(arr) <- c("Mean", "GM", "HM", "Med", "SV", "SD", "MAD", "MADm", "IQR", "Range")
    return(arr)
}

stats(c(6.7, 3.4, 0.8))

