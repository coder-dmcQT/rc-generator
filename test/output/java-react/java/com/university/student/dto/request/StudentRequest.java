package com.university.student.dto.request;
import lombok.Data;
import jakarta.validation.constraints.Size;
import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Email;
@Data
public class StudentRequest {
private Long id;
@Size(min = 8, max = 15, message = "Student ID must be 8-15 characters")
private String studentId;
@Size(max = 60, message = "Full name cannot exceed 60 characters")
@NotBlank(message = "University email is required")
private String fullName;
@Email(message = "Invalid email format (e.g., name@westernu.edu)")
@NotBlank(message = "University email is required")
private String email;
@Size(max = 30, message = "Major cannot exceed 30 characters")
private String major;
@NotBlank(message = "Family location is required")
private String location;
}