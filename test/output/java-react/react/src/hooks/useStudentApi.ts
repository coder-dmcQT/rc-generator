
    import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { studentService } from '../services/studentService';
import { Student, StudentCreateRequest, StudentFilterParams } from '../types/Student.types';

export const useStudentApi = () => {
  const queryClient = useQueryClient();

  const useGetStudents = (params?: StudentFilterParams) => {
    return useQuery({
      queryKey: ['students', params],
      queryFn: () => studentService.getStudents(params),
    });
  };

  const useGetStudentById = (id: number) => {
    return useQuery({
      queryKey: ['student', id],
      queryFn: () => studentService.getStudentById(id),
      enabled: !!id, 
    });
  };

  const useCreateStudent = () => {
    return useMutation({
      mutationFn: (student: StudentCreateRequest) => studentService.createStudent(student),
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: ['students'] }); // 刷新列表
      },
    });
  };

  const useUpdateStudent = () => {
    return useMutation({
      mutationFn: ({ id, student }: { id: number; student: StudentCreateRequest }) => 
        studentService.updateStudent(id, student),
      onSuccess: (_, { id }) => {
        queryClient.invalidateQueries({ queryKey: ['students'] });
        queryClient.invalidateQueries({ queryKey: ['student', id] });
      },
    });
  };

  const useDeleteStudent = () => {
    return useMutation({
      mutationFn: (id: number) => studentService.deleteStudent(id),
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: ['students'] });
      },
    });
  };

  return {
    useGetStudents,
    useGetStudentById,
    useCreateStudent,
    useUpdateStudent,
    useDeleteStudent,
  };
};
    