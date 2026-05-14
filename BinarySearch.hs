binarySearch :: Int -> [Int] -> Int
binarySearch _ [] = -1
binarySearch t xs =
  let mid = length xs `div` 2
      value = xs !! mid
  in if xs !! mid < t
     then
       let result = binarySearch t (snd $ splitAt (mid + 1) xs)
       in if result == -1
          then -1
          else (mid + 1) + result
     else if value == t
          then mid
          else binarySearch t (fst $ splitAt mid xs)

main :: IO ()
main = do
  let ret1 = binarySearch 124 [1, 4, 8, 22, 93, 103, 124, 125]
  let ret2 = binarySearch 67 [1, 4, 8, 22, 93, 103, 124, 125]
  putStrLn (show ret1)
  putStrLn (show ret2)
