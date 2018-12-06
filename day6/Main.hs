
main = interact $ func . head . words

func :: String -> String
func = 