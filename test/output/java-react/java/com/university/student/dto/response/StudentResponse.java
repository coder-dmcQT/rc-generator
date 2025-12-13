package com.university.student.dto.response;
import lombok.Builder;
import lombok.Data;
@Data
@Builder
public class StudentResponse {
private Long id;
private String studentId;
private String fullName;
private String email;
private String major;
private String location;
}