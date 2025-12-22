export interface Student {
id: number;
studentId:string| null;
fullName:string| null;
email:string| null;
major:string| null;
location:string| null;
}
export interface StudentCreateRequest {
id: number;
studentId:string| null;
fullName:string| null;
email:string| null;
major:string| null;
location:string| null;
}
export interface StudentFilterParams {
studentId:string| null;
email:string| null;
}
export interface ApiResponse<T>{
code: number;
message: string;
data: T;
}