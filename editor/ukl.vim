" Vim syntax file
" Language: Unknown Lang

" Usage Instructions
" Put this file in .vim/syntax/ukl.vim
" and add in your .vimrc file the next line:
" autocmd BufRead,BufNewFile *.ukl set filetype=ukl

if exists("b:current_syntax")
  finish
endif

set iskeyword=a-z,A-Z,-,*,_,!,@

syntax keyword uklTodos TODO FIXME NOTE

" Language keywords
syntax keyword uklKeywords let func import

" Comments
syntax region uklCommentLine start="//" end="$"          contains=uklTodos
syntax region uklMultiCommentLine start="/\*" end="\*/"  contains=uklTodos

" String literals
syntax region uklString start=/\v"/ skip=/\v\\./ end=/\v"/ contains=uklEscapes

" Char literals
syntax region uklChar start=/\v'/ skip=/\v\\./ end=/\v'/ contains=uklEscapes

" Escape literals \n, \r, ....
syntax match uklEscapes display contained "\\[nrt0\\\"']"

" Function definitions, matches the word 'func' followed by a word
syntax region uklFuncDef start="func" end=/\v\w+/ contains=uklFuncName

" Function name
syntax match uklFuncName display contained /\v\w+/

" Number literals
syntax match uklNumber display contained /\v[0-9]+/

" Type names the compiler recognizes
syntax keyword uklTypeNames u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize

" Set highlights
highlight default link uklTodos Todo
highlight default link uklKeywords Keyword
highlight default link uklCommentLine Comment
highlight default link uklMultiCommentLine Comment
highlight default link uklString String
highlight default link uklNumber Statement
highlight default link uklTypeNames Type
highlight default link uklChar Character
highlight default link uklEscapes SpecialChar
highlight default link uklFuncDef Function

let b:current_syntax = "ukl"

