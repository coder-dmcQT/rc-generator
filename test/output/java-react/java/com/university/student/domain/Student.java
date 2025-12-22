package com.university.student.domain;
import jakarta.persistence.*;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;
@Entity
@Table(name = "students")
@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
public class Student {
@Id
@GeneratedValue(strategy = GenerationType.IDENTITY)
private Long id;
@Column(length = 15, nullable = false, unique = true)
private String studentId;
@Column(length = 15, nullable = false)
private String fullName;
@Column(length = 100, nullable = false, unique = true)
private String email;
@Column(length = 30)
private String major;
@Column(length = 100)
private String location;
}