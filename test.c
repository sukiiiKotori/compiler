float add(float a, float b){
    float x=0.5;
    return a+b+x;
}

int main(){
    //float arr[4]={0.1,0.2};
    float f1 = 0x1.921fb6p+1;
    float f2 = 2.+0.4;
    float f = add(add(f1,f2),f2) ;
    return 0;
}