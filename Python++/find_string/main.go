package main

import (
	"io/ioutil"
	"log"
	"os"
	"strings"  
)

func main() {
	// 确保提供了足够的命令行参数
	if len(os.Args) != 4 {
		log.Fatal("Usage: program <inputString> <char1> <char2>")
	}
  
	// 读取命令行参数  
	inputString := os.Args[1]  
	char1 := string(os.Args[2][0]) // 只需要字符的第一个字节
	char2 := string(os.Args[3][0]) // 只需要字符的第一个字节

	// 在输入字符串中查找字符1和字符2的位置
	start := strings.Index(inputString, char1)
	end := strings.Index(inputString[start+1:], char2) // 从字符1后面开始查找字符2
	if start == -1 || end == -1 {
		log.Fatal("One or both characters not found in the input string.")
	}
	end += start + 1 // 调整结束位置以匹配原始字符串的索引

	// 提取字符1和字符2之间的子字符串
	substring := inputString[start+1 : end]

	// 将子字符串写入temp.txt文件
	err := ioutil.WriteFile("temp.txt", []byte(substring), 0644)
	if err != nil {
		log.Fatal(err)
	}
}