
    import React, { useState } from 'react';
import { Box, Typography, Tabs, Tab } from '@mui/material';
import StudentList from '../../components/student/StudentList/StudentList';
import StudentForm from '../../components/student/StudentForm/StudentForm';
import { useStudentApi } from '../../hooks/useStudentApi';
import { Student, StudentCreateRequest } from '../../types/Student.types';

const StudentPage: React.FC = () => {
  const [activeTab, setActiveTab] = useState(0);
  const [editingStudent, setEditingStudent] = useState<Student | null>(null);
  const { useCreateStudent, useUpdateStudent } = useStudentApi();
  const { mutate: createStudent, isPending: isCreating } = useCreateStudent();
  const { mutate: updateStudent, isPending: isUpdating } = useUpdateStudent();

  // 处理表单提交（新增/编辑）
  const handleFormSubmit = (student: StudentCreateRequest) => {
    if (editingStudent) {
      updateStudent({ id: editingStudent.id, student });
    } else {
      createStudent(student);
    }
    setActiveTab(0); // 提交后切回列表
    setEditingStudent(null); // 清空编辑状态
  };

  // 处理编辑
  const handleEdit = (student: Student) => {
    setEditingStudent(student);
    setActiveTab(1);
  };

  // 处理查看
  const handleView = (student: Student) => {
    // 跳转到详情页（React Router）
    window.location.href = `/student/${student.id}`;
  };

  return (
    <Box sx={{ p: 3, maxWidth: 1200, margin: '0 auto' }}>
      <Typography variant="h4" gutterBottom>
        Student Management
      </Typography>
      <Tabs value={activeTab} onChange={(_, newValue) => setActiveTab(newValue)} sx={{ mb: 2 }}>
        <Tab label="Student List" />
        <Tab label={editingStudent ? 'Edit Student' : 'Add Student'} />
      </Tabs>
      {activeTab === 0 && (
        <StudentList 
          onEdit={handleEdit} 
          onView={handleView} 
        />
      )}
      {activeTab === 1 && (
        <StudentForm
          initialData={editingStudent}
          onSubmit={handleFormSubmit}
          isSubmitting={isCreating || isUpdating}
        />
      )}
    </Box>
  );
};

export default StudentPage;
    