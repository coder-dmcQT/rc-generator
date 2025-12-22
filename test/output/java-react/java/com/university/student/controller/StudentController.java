package com.university.student.controller;
import com.university.student.dto.api.ApiResponse;
import com.university.student.dto.response.StudentResponse;
import com.university.student.dto.request,StudentRequest;
import com.university.student.service.StudentService;
import jakarta.validation.Valid;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.*;
import java.util.List;
@Slf4j
@RestController
@RequestMapping("/api/v1/student")
@RequiredArgsConstructor
public class StudentController{
private final StudentService StudentService;
@PostMapping
@ResponseStatus(HttpStatus.CREATED)
public ApiResponse<StudentResponse> createStudent(@Valid @RequestBody StudentRequest request) {
log.info("Initiating Student for ID {}", request.getStudentId());
StudentResponse createdStudent = StudentService;
return ApiResponse.success(Student created successfully, createdStudent);
}
@GetMapping
@ResponseStatus(HttpStatus.OK)
public ApiResponse<List<StudentResponse>> getAllStudent(){
log.info("Retrieving all Students");
List<StudentResponse> resp = StudentService.getAllStudents;
return ApiResponse.success(resp);
}
@GetMapping("/{id}")
@ResponseStatus(HttpStatus.OK)
public ApiResponse<StudentResponse> getStudentById(@PathVariable Long id){
log.info("Retrieving Student by id");
StudentResponse resp = StudentService.getStudentById(id);
return ApiResponse.success(resp);
}
@PutMapping("/id")
@ResponseStatus(HttpStatus.OK)
public ApiResponse<StudentResponse> updateStudent(@PathVariable Long id, @Valid @RequestBody StudentRequest request) {
log.info("Updating Student by id");
StudentResponse resp = StudentService.updateStudent(id, request);
return ApiResponse.success(resp);
}
@DeleteMapping("/{id}")
@ResponseStatus(HttpStatus.OK)
public ApiResponse<void> deleteStudent(@PathVariable Long id){
StudentService.deleteStudent(id);
return ApiResponse.success(null);
}
}