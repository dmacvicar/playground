
listHead :: [a] -> a
listHead [] = error "Empty list don't have a head"
listHead (x:_) = x

listLen :: (Num b) => [a] -> b
listLen [] = 0
listLen (x:xs) = 1 + (listLen xs)

listSum :: (Num a) => [a] -> a
listSum [] = 0
listSum (x:xs) = x + (listSum xs)

main :: IO ()

main = do {
    putStrLn (show (listHead [4,1,2,3]));
    putStrLn (show (listLen [4,2,3]));
    putStrLn (show (listSum [4,2,3]));
}

