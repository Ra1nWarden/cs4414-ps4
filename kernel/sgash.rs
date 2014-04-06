/* kernel::sgash.rs */
#[allow(unused_imports)];
use core::*;
use core::str::*;
use core::option::{Some, Option, None}; 
use core::iter::Iterator;
use kernel::*;
use super::super::platform::*;
use kernel::memory::Allocator;

pub static mut buff_str : c_string = c_string {
    start_ptr: 0 as *mut u8,
    next_index: 0,
    max_length: 0
};

pub static mut file_sys: Linked_list = Linked_list {
    start: 0 as *mut List_Node,
    end: 0 as *mut List_Node,
    length: 0
};

pub static mut current_dir: c_string = c_string {
    start_ptr: 0 as *mut u8,
    next_index: 0,
    max_length: 0
};

pub static mut next_pointer : *mut u8 = 0x000000 as *mut u8;
pub static mut all_length : uint = 0;

pub fn putchar(key: char) {
    unsafe {
	/*
	 * We need to include a blank asm call to prevent rustc
	 * from optimizing this part out
	 */
	asm!("");
	io::write_char(key, io::UART0);
    }
}

fn putstr(msg: &str) {
    for c in slice::iter(as_bytes(msg)) {
	putchar(*c as char);
    }	
}

pub unsafe fn drawstr(msg: &str) {
    for c in slice::iter(as_bytes(msg)) {
	drawchar(*c as char);
    }
}

unsafe fn drawchar(x: char)
{
    io::restore();
    if x == '\n' {
	io::CURSOR_Y += io::CURSOR_HEIGHT;
	io::CURSOR_X = 0u32;
    } else {
	io::draw_char(x);	
	io::CURSOR_X += io::CURSOR_WIDTH;
    }
    io::backup();
    io::draw_cursor();
}

unsafe fn backspace()
{
    io::restore();
    if (io::CURSOR_X >= io::CURSOR_WIDTH) { 
	io::CURSOR_X -= io::CURSOR_WIDTH;
	io::draw_char(' ');
    }
    io::backup();
    io::draw_cursor();
}

pub unsafe fn interpret(mut cmd_str: c_string) {
    let (x, y) = cmd_str.split(' ');
    let mut cmd = x;
    let mut args = y;
    if (cmd.eq(&"echo")) {
        args.print();
    }
    else if (cmd.eq(&"ls")) {
        drawstr(&"ls");
    }
    else if (cmd.eq(&"cat")) {
        drawstr(&"cat");
    }
    else if (cmd.eq(&"cd")) {
        drawstr(&"cd");
    }
    else if (cmd.eq(&"rm")) {
        drawstr(&"rm");
    }
    else if (cmd.eq(&"mkdir")) {
        drawstr(&"mkdir");
    }
    else if (cmd.eq(&"pwd")) {
        current_dir.print();
    }
    else if (cmd.eq(&"wr")) {
        drawstr(&"wr");
    }
    else {
        drawstr(&"invalid command: ");
        cmd.print();
        drawstr(&"\n");
    }
}

pub unsafe fn parsekey(x: char) {
    let x = x as u8;
    // Set this to false to learn the keycodes of various keys!
    // Key codes are printed backwards because life is hard

    match x { 
	13		=>	{ 
            putstr(&"\n");
            drawstr(&"\n");
            interpret(buff_str);
	    putstr(&"\nsgash>");
	    drawstr(&"\nsgash>");
            buff_str.clear();
	}
	127		=>	{ 
	    putchar('');
	    putchar(' ');
	    putchar(''); 
	    backspace();
            buff_str.backSpace();
	}
	_		=>	{ 
	    if io::CURSOR_X < io::SCREEN_WIDTH-io::CURSOR_WIDTH {
		putchar(x as char);
		drawchar(x as char);
                buff_str.addChar(x as u8);
	    }
	}
    }
}

fn screen() {
    putstr(&"\n                                                               "); 
    putstr(&"\n                                                               ");
    putstr(&"\n                       7=..~$=..:7                             "); 
    putstr(&"\n                  +$: =$$$+$$$?$$$+ ,7?                        "); 
    putstr(&"\n                  $$$$$$$$$$$$$$$$$$Z$$                        ");
    putstr(&"\n              7$$$$$$$$$$$$. .Z$$$$$Z$$$$$$                    ");
    putstr(&"\n           ~..7$$Z$$$$$7+7$+.?Z7=7$$Z$$Z$$$..:                 ");
    putstr(&"\n          ~$$$$$$$$7:     :ZZZ,     :7ZZZZ$$$$=                ");
    putstr(&"\n           Z$$$$$?                    .+ZZZZ$$                 ");
    putstr(&"\n       +$ZZ$$$Z7                         7ZZZ$Z$$I.            "); 
    putstr(&"\n        $$$$ZZZZZZZZZZZZZZZZZZZZZZZZI,    ,ZZZ$$Z              "); 
    putstr(&"\n      :+$$$$ZZZZZZZZZZZZZZZZZZZZZZZZZZZ=    $ZZ$$+~,           "); 
    putstr(&"\n     ?$Z$$$$ZZZZZZZZZZZZZZZZZZZZZZZZZZZZI   7ZZZ$ZZI           "); 
    putstr(&"\n      =Z$$+7Z$$7ZZZZZZZZ$$$$$$$ZZZZZZZZZZ  ~Z$?$ZZ?            ");	 
    putstr(&"\n    :$Z$Z...$Z  $ZZZZZZZ~       ~ZZZZZZZZ,.ZZ...Z$Z$~          "); 
    putstr(&"\n    7ZZZZZI$ZZ  $ZZZZZZZ~       =ZZZZZZZ7..ZZ$?$ZZZZ$          "); 
    putstr(&"\n      ZZZZ$:    $ZZZZZZZZZZZZZZZZZZZZZZ=     ~$ZZZ$:           "); 
    putstr(&"\n    7Z$ZZ$,     $ZZZZZZZZZZZZZZZZZZZZ7         ZZZ$Z$          "); 
    putstr(&"\n   =ZZZZZZ,     $ZZZZZZZZZZZZZZZZZZZZZZ,       ZZZ$ZZ+         "); 
    putstr(&"\n     ,ZZZZ,     $ZZZZZZZ:     =ZZZZZZZZZ     ZZZZZ$:           "); 
    putstr(&"\n    =$ZZZZ+     ZZZZZZZZ~       ZZZZZZZZ~   =ZZZZZZZI          "); 
    putstr(&"\n    $ZZ$ZZZ$$Z$$ZZZZZZZZZ$$$$   IZZZZZZZZZ$ZZZZZZZZZ$          "); 
    putstr(&"\n      :ZZZZZZZZZZZZZZZZZZZZZZ   ~ZZZZZZZZZZZZZZZZZ~            "); 
    putstr(&"\n     ,Z$$ZZZZZZZZZZZZZZZZZZZZ    ZZZZZZZZZZZZZZZZZZ~           "); 
    putstr(&"\n     =$ZZZZZZZZZZZZZZZZZZZZZZ     $ZZZZZZZZZZZZZZZ$+           "); 
    putstr(&"\n        IZZZZZ:.                        . ,ZZZZZ$              "); 
    putstr(&"\n       ~$ZZZZZZZZZZZ                 ZZZZ$ZZZZZZZ+             "); 
    putstr(&"\n           Z$ZZZ. ,Z~               =Z:.,ZZZ$Z                 "); 
    putstr(&"\n          ,ZZZZZ..~Z$.             .7Z:..ZZZZZ:                ");
    putstr(&"\n          ~7+:$ZZZZZZZZI=:.   .,=IZZZZZZZ$Z:=7=                ");
    putstr(&"\n              $$ZZZZZZZZZZZZZZZZZZZZZZ$ZZZZ                    ");
    putstr(&"\n              ==..$ZZZ$ZZZZZZZZZZZ$ZZZZ .~+                    ");
    putstr(&"\n                  I$?.?ZZZ$ZZZ$ZZZI =$7                        ");
    putstr(&"\n                       $7..I$7..I$,                            ");
    putstr(&"\n"); 
    putstr(&"\n _                     _     _                         _  ");
    putstr(&"\n| |                   (_)   | |                       | | ");
    putstr(&"\n| | ____ ___  ____     _____| |_____  ____ ____  _____| | ");
    putstr(&"\n| |/ ___) _ \\|  _ \\   |  _   _) ___ |/ ___)  _ \\| ___ | | ");
    putstr(&"\n| | |  | |_| | | | |  | |  \\ \\| ____| |   | | | | ____| | ");
    putstr(&"\n|_|_|  \\____/|_| |_|  |_|   \\_\\_____)_|   |_| |_|_____)__)\n\n");
}

pub unsafe fn init() {
    let (x, y) = heap.alloc(20);
    buff_str = c_string::new(10);
    file_sys = Linked_list::new();
    current_dir = c_string::new(10);
    current_dir.addStr(&"home");
    current_dir.print();
    screen();
    putstr(&"sgash>");
}

// c_string struct

struct c_string {
    start_ptr: *mut u8,
    next_index: uint,
    max_length: uint
}

impl c_string {
    pub unsafe fn new(size: uint) -> c_string {
        let (x, y) = heap.alloc(size);
        let mut retVal = c_string {
            start_ptr: x,
            next_index: 0,
            max_length: y
        };
        next_pointer = (next_pointer as uint + size) as *mut u8;
        *((retVal.start_ptr as uint + retVal.next_index) as *mut char) = '\0';
        retVal
    }
    pub unsafe fn from_str(fname: &str) -> c_string {
        let (x, y) = heap.alloc(255);
        let mut retVal = c_string {
            start_ptr: x,
            next_index: 0,
            max_length: y
        };
        next_pointer = (next_pointer as uint + 10) as *mut u8;
        for c in slice::iter(as_bytes(fname)) {
            retVal.addChar(*c as u8);
        }
        retVal
    }
    unsafe fn addStr(&mut self, addStr: &str) -> bool {
        let mut result = false;
        for c in slice::iter(as_bytes(addStr)) {
            result = self.addChar(*c);
            if (result == false) {
                break;
            }
        }
        result
    }
    unsafe fn addChar(&mut self, c: u8) -> bool {
        if (self.next_index == self.max_length) {
            false
        }
        else {
            *((self.start_ptr as uint + self.next_index) as *mut u8) = c;
            self.next_index = self.next_index + 1;
            *((self.start_ptr as uint + self.next_index) as *mut char) = '\0';
            true
        }
    }
    unsafe fn clear(&mut self) -> bool {
        self.next_index = 0;
        *(self.start_ptr as *mut char) = '\0';
        true
    }
    unsafe fn backSpace(&mut self) -> bool {
        if (self.next_index == 0) {
            false
        }
        else {
            self.next_index -= 1;
            *((self.start_ptr as uint + self.next_index) as *mut char) = '\0';
            true
        }
    }
    unsafe fn print(&mut self) {
        let mut i : uint = 0;
        while (i < self.next_index) {
            let c = (self.start_ptr as uint + i) as *mut char;
            if (*c == '\0') {
                break;
            }
            else {
                // putchar(*c);
                drawchar(*c);
            }
            i += 1;
        }
    }
    unsafe fn eq(&mut self, cmp_str: &str) -> bool{
        let mut running_index = self.start_ptr;
        let mut result = true;
        for itr in slice::iter(as_bytes(cmp_str)) {
            if (*itr as char != *running_index as char) {
                result = false;
                break;
            }
            running_index = (running_index as uint + 1) as *mut u8;
        }
        if ((*running_index) as char != '\0') {
            result = false;
        }
        result
    }
    unsafe fn split(&self, c: char) -> (c_string, c_string) {
        let mut start_pointer: uint = self.start_ptr as uint;
        let (x, y) = heap.alloc(20);
        let mut first = c_string::new(20);
        let mut second = c_string::new(20);
        let mut found = false;
        loop {
            if (*(start_pointer as *char) == '\0') {
                return (first, second);
            }
            else if (*(start_pointer as *u8) == c as u8) {
                found = true;
            }
            else if (!found) {
                first.addChar(*(start_pointer as *u8));
            }
            else if (found) {
                second.addChar(*(start_pointer as *u8));
            }
            start_pointer += 1;
        }
    }
}

// Linked_list implementation

pub static null_ptr: uint = 0xFFFFFF;

struct List_Node {
    name: *c_string,
    prev: *mut List_Node,
    next: *mut List_Node,
    parent: *mut List_Node,
    children: *Linked_list
}

impl List_Node {
    pub fn newFile(fname: c_string) -> List_Node {
        let retVal = List_Node {
            name: &fname,
            prev: null_ptr as *mut List_Node,
            next: null_ptr as *mut List_Node,
            parent: null_ptr as *mut List_Node,
            children: null_ptr as *Linked_list
        };
        retVal
    }
    pub unsafe fn newDir(fname: c_string) -> List_Node {
        let retVal = List_Node {
            name: &fname,
            prev: null_ptr as *mut List_Node,
            next: null_ptr as *mut List_Node,
            parent: null_ptr as *mut List_Node,
            children: &(Linked_list::new()) as *Linked_list
        };
        retVal
    }
    fn isFile(&mut self) -> bool {
        if (self.children as uint == 0xFFFFFF) {
            true
        }
        else {
            false
        }
    }
}

struct Linked_list {
    start: *mut List_Node,
    end: *mut List_Node,
    length: uint
}

impl Linked_list {
    pub fn new() -> Linked_list {
        let retVal = Linked_list {
            start: null_ptr as *mut List_Node,
            end: null_ptr as *mut List_Node,
            length: 0
        };
        retVal
    }
    unsafe fn add_Node(&mut self, x: *mut List_Node) {
        (*self.end).next = x;
        self.end = x;
        self.length += 1;
    }
}
