module Main where

import Data.List (splitOn)
import System.Environment (getArgs)

-- ==================================================================
-- Setup
-- ==================================================================

parseRanges :: String -> [String]
parseRanges = splitOn ","

-- ==================================================================
-- Part 1A
--
-- This solution was the first that I tried. It worked fine on the
-- example input but took nearly 16 minutes on the real input.
-- ==================================================================

countDigits :: Int -> Int
countDigits n
  | abs n == 0 = 1
  | otherwise = 1 + (floor $ logBase 10 (fromIntegral (abs n) + 0.5))

hasRepeatedHalf :: Int -> Bool
hasRepeatedHalf n
  | odd digitCount = False
  | otherwise = lhs == rhs
  where digitCount = countDigits n
        halfDigitCount = div digitCount 2
        divisor = 10 ^ halfDigitCount
        lhs = div n divisor
        rhs = mod n divisor

invalidIdsInRange1A :: String -> [Int]
invalidIdsInRange1A range =
  let [start, end] = map read $ splitOn "-" range
   in [id | id <- [start..end], hasRepeatedHalf id]

solvePart1A :: String -> Int
solvePart1A = foldl (+) 0 . concatMap invalidIdsInRange1A . parseRanges

-- ==================================================================
-- Part 1B
--
-- This solution to part 1 is much faster and runs in about 1 second.
-- ==================================================================

splitInHalf :: Int -> (Int, Int)
splitInHalf id = (lhs, rhs)
  where digitCount = countDigits id
        halfDigitCount = div digitCount 2
        lhs = div id (10 ^ halfDigitCount)
        rhs = mod id (10 ^ halfDigitCount)

isInvalidId :: Int -> Bool
isInvalidId id = lhs == rhs
  where (lhs, rhs) = splitInHalf id

nextInvalidIdFrom :: Int -> Int
nextInvalidIdFrom id = nextId
  where (lhs, rhs) = splitInHalf id
        digitCount = countDigits id
        nextId
          | odd digitCount = nextInvalidIdFrom $ 10 ^ digitCount
          | lhs > rhs = lhs * (10 ^ countDigits lhs) + lhs
          | lhs <= rhs = lhs' * (10 ^ countDigits lhs') + lhs'
            where lhs' = lhs + 1

invalidIdsInRange1B :: String -> [Int]
invalidIdsInRange1B range
  | isInvalidId start = start : generateFrom (nextInvalidIdFrom start)
  | otherwise = generateFrom (nextInvalidIdFrom start)
  where [start, end] = map read $ splitOn "-" range
        generateFrom invalidId
          | invalidId > end = []
          | otherwise = invalidId : generateFrom (nextInvalidIdFrom invalidId)

solvePart1B :: String -> Int
solvePart1B = foldl (+) 0 . concatMap invalidIdsInRange1B . parseRanges

-- ==================================================================
-- Part 2
-- ==================================================================

solvePart2 :: String
solvePart2 = "solvePart2"

-- ==================================================================
-- Main
-- ==================================================================

main :: IO ()
main = do
  args <- getArgs
  input <- readFile "input/input.txt"
  case args of
    ["1A"] -> print $ solvePart1A input
    ["1B"] -> print $ solvePart1B input
    ["2"] -> putStrLn solvePart2
    _     -> print input

