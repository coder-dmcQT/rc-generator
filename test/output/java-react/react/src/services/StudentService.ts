import apiClient from "./apiClient"
import {Student, StudentCreateRequest, StudentFilterParams} from "../types/Student.types"
export const studentService = {
getStudents: (params?: StudentFilterParams): Promise<Student[]> => {
return apiClient.get('/student', {params});
},
getStudentById: (id: number): Promise<Student> => {
return apiClient.get(`/student/${id}`);
},
createStudent: (student: StudentCreateRequest): Promise<Student> => {
return apiClient.post('/student', student);
},
updateStudent: (id: number, student: StudentCreateRequest): Promise<Student> => {
return apiClient.put(`/student/${id}`, student)
},
deleteStudent: (id: number): Promise<void> => {
return apiClient.delete(`/student/${id}`);
},
}