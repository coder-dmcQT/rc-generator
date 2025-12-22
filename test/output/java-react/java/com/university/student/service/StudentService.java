package com.university.student.service;
import com.university.student.domain.Student;
import com.university.student.repository.StudentRepository;
import com.university.student.dto.request.StudentRequest;
import com.university.student.dto.response.StudentResponse;
import com.university.student.Service.StudentService;
public interface StudentService {
StudentResponse createStudent(StudentRequest Sr);
List<StudentResponse> getAllStudents();
StudentResponse getStudentById(Long id);
StudentResponse updateStudent(Long id, StudentRequest Sr);
void deleteStudent(Long id);
}