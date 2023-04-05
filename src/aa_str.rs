// Declaring variables of Ascii art
static A: [&str; 3]= ["▄▀▀▄", "█▀▀█", "▀░░▀"];

static B: [&str; 3]= ["█▀▀▄", "█▀▀█", "▀▀▀░"];

static C_0: [&str; 3]= ["█▀▀▀", "█░░░", "▀▀▀▀"];
static C_1: [&str; 3]= ["█▀▀", "█░░", "▀▀▀"];

static D: [&str; 3]= ["█▀▀▄", "█░░█", "▀▀▀░"];

static E: [&str; 3]= ["█▀▀", "█▀▀", "▀▀▀"];

static F: [&str; 3]= ["█▀▀", "█▀▀", "▀░░"];

static G: [&str; 3]= ["█▀▀▀", "█░▀█", "▀▀▀▀"];

static H: [&str; 3]= ["█░░█", "█▀▀█", "▀░░▀"];

static I_0: [&str; 3]= ["▀█▀", "░█░", "▀▀▀"];
static I_1: [&str; 3]= ["█", "█", "▀"];

static J_0: [&str; 3]= ["░░░█", "▄░░█", "░▀▀░"];
static J_1: [&str; 3]= ["░░█", "▄░█", "░▀░"];

static K_0: [&str; 3]= ["█░░▄", "█▀▀▄", "▀░░▀"];
static K_1: [&str; 3]= ["█░▄", "█▀▄", "▀░▀"];

static L: [&str; 3]= ["█░░", "█░░", "▀▀▀"];

static M: [&str; 3]= ["█▄░▄█", "█░▀░█", "▀░░░▀"];

static N: [&str; 3]= ["█▄░█", "█░▀█", "▀░░▀"];

static O_0: [&str; 3]= ["█▀▀█", "█░░█", "▀▀▀▀"];
static O_1: [&str; 3]= ["█▀█", "█░█", "▀▀▀"];

static P: [&str; 3]= ["█▀█", "█▀▀", "▀░░"];

static Q: [&str; 3]= [ "█▀▀█░", "█░▀█▄", "▀▀▀▀░"];

static R: [&str; 3]= ["█▀▀▄", "█▀▀▄", "▀░░▀"];

static S: [&str; 3]= ["█▀▀▀", "▄▀▀█", "▀▀▀▀"];

static T: [&str; 3]= ["▀█▀", "░█░", "░▀░"];

static U_0: [&str; 3]= ["█░░█", "█░░█", "▀▀▀▀"];
static U_1: [&str; 3]= ["█░█", "█░█", "▀▀▀"];

static V: [&str; 3]= ["█░█", "█░█", "░▀░"];

static W: [&str; 3]= ["█░░░█", "█░█░█", "░▀░▀░"];

static X: [&str; 3]= ["█░█", "▄▀▄", "▀░▀"];

static Y: [&str; 3]= ["█░█", "░█░", "░▀░"];

static Z: [&str; 3]= ["▀▀█", "▄▀░", "▀▀▀"];

// Declaring Public variables
pub static EXCLAMATION: [&str; 3]= ["█", "▀", "▀"];

pub static PERIOD: [&str; 3]= ["░", "░", "▀"];

pub static SPACE: &str = "░";

pub static AA: [[[&str; 3]; 26]; 2]= [
    [A, B, C_0, D, E, F, G, H, I_0, J_0, K_0, L, M, N, O_0, P, Q, R, S, T, U_0, V, W, X, Y, Z],
    [A, B, C_1, D, E, F, G, H, I_1, J_1, K_1, L, M, N, O_1, P, Q, R, S, T, U_1, V, W, X, Y, Z]
];