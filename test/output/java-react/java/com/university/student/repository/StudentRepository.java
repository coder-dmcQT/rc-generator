package com.university.student.repository;
import java.util.List;
import java.util.Optional;
import org.springframework.data.repository.Repository;
import com.university.student.domain.Student;
public interface StudentRepository extends Repository<Student, Long>{
Optional<Student> findByStudentId(Long id, String studentId);
boolean existsByStudentId(Long id, String studentId);
Optional<Student> findByEmail(String email);
boolean existsByEmail(String email);
}