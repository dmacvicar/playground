main :: IO ()

main = do 
    putStrLn (show (factorial 4))

factorial :: Integer -> Integer
factorial 0 = 1
factorial x = x * (factorial (x - 1))

