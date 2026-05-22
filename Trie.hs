module Main where

-- https://en.wikipedia.org/wiki/Trie

-- A trie is an n-ary tree
--    fun fact: a radix tree is a compressed trie
data Trie = Trie Bool [(Char, Trie)]
  deriving (Show, Eq)

trieChildrenTraverse :: [(Char, Trie)] -> Char -> Maybe Trie
trieChildrenTraverse [] _ = Nothing
trieChildrenTraverse ((c, t):xs) c'
  | c == c' = Just t
  | otherwise   = trieChildrenTraverse xs c'

trieWordExists :: Trie -> String -> Bool
trieWordExists (Trie True _) [] = True
trieWordExists (Trie _ []) _ = False
trieWordExists (Trie isWord children) (s:search) =
  case (trieChildrenTraverse children s) of
    Nothing -> False
    Just t  -> if isWord && (search == []) then
                 True
               else trieWordExists t search

trieFindSnippet :: Trie -> String -> Maybe Trie
trieFindSnippet t [] = Just t
trieFindSnippet (Trie _ children) (s:search) = do
  child <- trieChildrenTraverse children s
  trieFindSnippet child search

trieWords :: Trie -> [String]
trieWords (Trie isWord children) =
  let here =
        if isWord then [""] else []

      below =
        [ c : suffix
        | (c, child) <- children
        , suffix <- trieWords child
        ]
  in here ++ below

-- find (<= n) next words
searchPrefix :: Trie -> String -> Int -> Maybe [String]
searchPrefix trie prefix n = do
  node <- trieFindSnippet trie prefix
  pure $ take n $ map (prefix ++) $ trieWords node

trieInsert :: Trie -> String -> Trie
trieInsert = undefined

trieDelete :: Trie -> String -> Trie
trieDelete = undefined

parseWordsIntoTrie :: [String] -> Trie
parseWordsIntoTrie = undefined

parseTextToWords :: String -> [String]
parseTextToWords = undefined

exampleTrie :: Trie
exampleTrie =
  Trie False
    [ ( 'c'
      , Trie False
          [ ( 'a'
            , Trie False
                [ ( 'r'
                  , Trie True
                      [ ( 't'
                        , Trie True []
                        )
                      ]
                  )
                , ( 't'
                  , Trie True []
                  )
                ]
            )
          ]
      )
    ]

main :: IO ()
main = do
  putStrLn $ show $ trieWordExists exampleTrie "cat"
  putStrLn $ show $ searchPrefix exampleTrie "ca" 3
