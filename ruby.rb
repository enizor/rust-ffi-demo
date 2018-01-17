# Simple function
def is_perfect(n)
    divisor = 1
    sum = 0
    while divisor * divisor < n do
        if n % divisor == 0
            sum += divisor
            if n / divisor != divisor
                sum += n / divisor
            end
        end
        divisor += 1
    end
    sum == 2*n
end

# Loop N times to find all the perfect numbers < N
n = 1
res = []
while n < ARGV[0].to_i do
    if is_perfect(n)
        res.push(n)
    end
    n += 1
end
puts res.size
