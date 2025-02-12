




type MathMarks = {
    marks: number[]
};


const llm_data = {
    marks: [10, 20, 30]
}


function type_data<T>(data: any) T {
    return data;
}
