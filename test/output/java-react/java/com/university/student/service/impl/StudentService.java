package com.university.student.service.impl;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import com.university.student.service.StudentService;
import com.university.student.domain.Student;
import com.university.student.repository.StudentRepository;
import com.university.student.dto.request.StudentRequest;
import com.university.student.dto.response.StudentResponse;
import com.university.student.Service.StudentService;
public class StudentServiceImpl implements StudentService {
@Override
@Transactional
public StudentResponse createStudent(StudentRequest Sr) {
StudentResponse StudentResponse = null;
return StudentResponse;
}
@Override
@Transactional
public List<StudentResponse> getAllStudents() {
List<StudentResponse> List<StudentResponse> = null;
return List<StudentResponse>;
}
@Override
@Transactional
public StudentResponse getStudentById(Long id) {
StudentResponse StudentResponse = null;
return StudentResponse;
}
@Override
@Transactional
public StudentResponse updateStudent(Long id, StudentRequest Sr) {
StudentResponse StudentResponse = null;
return StudentResponse;
}
@Override
@Transactional
public void deleteStudent(Long id) {
void Void = null;
return Void;
}
}