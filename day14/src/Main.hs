module Main where

import qualified Data.Sequence as Seq
import Data.List
import Data.Digits (digits)
import Data.Monoid

main :: IO ()
main = do
    putStrLn "Tests"
    putStrLn . intercalate "" $ map show $ part1 9
    putStrLn "5158916779"
    putStrLn . intercalate "" $ map show $ part1 5
    putStrLn "0124515891"
    putStrLn . intercalate "" $ map show $ part1 18
    putStrLn "9251071085"
    putStrLn . intercalate "" $ map show $ part1 2018
    putStrLn "5941429882"
    putStrLn "Output"
    putStrLn . intercalate "" $ map show $ part1 824501

-- the issue i am running into is that we cannot see the start of the list we create
-- on any recursive call but the first basically meaning that we cannot do it this way
-- unless there is a way to pass both but then i cannot do it with infinite lists
recipe' y x xs =
        newDigs ++ recipe' recipe1 recipe2 newSeq
        where
            score1 = xs `Seq.index` y
            score2 = xs `Seq.index` x
            newDigs = digits 10 (score1 + score2)
            newSeq = xs <> Seq.fromList newDigs
            recipe1 = ((y + score1 + 1) `mod` (Seq.length newSeq))
            recipe2 = ((x + score2 + 1) `mod` (Seq.length newSeq))
        
part1 n = take 10 (drop n list)
        where
            list =  3 : 7 : recipe' 0 1 sq
            sq = Seq.fromList [3, 7]

